mod buffer;
mod math;
mod text;

pub use math::Vec2f;
pub use text::TextRenderer;

macro_rules! include_shader {
    ($name:literal, $file_name:literal) => {{
        wgpu::ShaderModuleDescriptor {
            label: Some($name),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/",
                $file_name,
            )))),
        }
    }};
}

use include_shader;

pub struct WgpuState {
    _instance: wgpu::Instance,
    surface: wgpu::Surface,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
}

impl WgpuState {
    pub fn create(window: &winit::window::Window) -> Self {
        use wgpu::*;

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            dx12_shader_compiler: Dx12Compiler::Fxc,
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::downlevel_defaults().using_resolution(adapter.limits()),
            },
            None,
        ))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        Self {
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
            surface_config,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.surface_config.width = new_width.max(1);
        self.surface_config.height = new_height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    #[inline]
    pub fn get_back_buffer(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    #[inline]
    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
    }

    #[inline]
    pub fn submit_encoder(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit([encoder.finish()]);
    }

    fn load_image<R: std::io::BufRead + std::io::Seek>(
        &self,
        reader: R,
        label: Option<&str>,
        srgb: bool,
    ) -> wgpu::Texture {
        use image::ImageFormat;
        use wgpu::util::DeviceExt;
        use wgpu::*;

        let img = image::load(reader, ImageFormat::Png).unwrap();
        let img = img.to_rgba8();

        let desc = TextureDescriptor {
            label,
            size: Extent3d {
                width: img.width(),
                height: img.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: if srgb {
                TextureFormat::Rgba8UnormSrgb
            } else {
                TextureFormat::Rgba8Unorm
            },
            usage: TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        self.device
            .create_texture_with_data(&self.queue, &desc, img.as_raw())
    }

    fn create_pipeline(
        &self,
        name: &str,
        shader: &wgpu::ShaderModule,
        bind_group_layout: &wgpu::BindGroupLayout,
        vs_input_layout: &[wgpu::VertexBufferLayout<'_>],
        blend: Option<wgpu::BlendState>,
    ) -> (wgpu::PipelineLayout, wgpu::RenderPipeline) {
        use wgpu::*;

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(name),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: shader,
                    entry_point: "vs_main",
                    buffers: vs_input_layout,
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: shader,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: self.surface_config.format,
                        blend,
                        write_mask: ColorWrites::all(),
                    })],
                }),
                multiview: None,
            });

        (pipeline_layout, pipeline)
    }
}

const VGA_SHADER_CODE: wgpu::ShaderModuleDescriptor<'_> = include_shader!("VGA shader", "vga.wgsl");

pub struct Vga {
    _shader: wgpu::ShaderModule,
    pipeline: wgpu::RenderPipeline,
}

impl Vga {
    pub fn new(wgpu_state: &WgpuState) -> Self {
        use wgpu::*;

        let shader = wgpu_state.device.create_shader_module(VGA_SHADER_CODE);
        let pipeline = wgpu_state
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("VGA pipeline"),
                layout: None,
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu_state.surface_config.format.into())],
                }),
                multiview: None,
            });

        Self {
            _shader: shader,
            pipeline,
        }
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, render_target: &wgpu::TextureView) {
        use wgpu::*;

        let mut vga_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("VGA pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: render_target,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        vga_pass.set_pipeline(&self.pipeline);
    }
}
