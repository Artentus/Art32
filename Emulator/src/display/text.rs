mod atlas;
use atlas::*;

use super::buffer::*;
use super::math::Vec2f;
use super::{include_shader, WgpuState};
use bytemuck::{Pod, Zeroable};
use wgpu::*;

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct Vertex {
    position: Vec2f,
    uv: Vec2f,
    color: [u8; 4],
    px_range: f32,
}

const MAX_VERTEX_COUNT: usize = (u16::MAX as usize) + 1;
const BATCH_SIZE: usize = MAX_VERTEX_COUNT / 4;

#[allow(clippy::identity_op)]
const INDICES: [u16; BATCH_SIZE * 6] = {
    let mut indices = [0; BATCH_SIZE * 6];
    let mut i = 0;
    while i < BATCH_SIZE {
        indices[i * 6 + 0] = (i as u16) * 4 + 0;
        indices[i * 6 + 1] = (i as u16) * 4 + 1;
        indices[i * 6 + 2] = (i as u16) * 4 + 2;
        indices[i * 6 + 3] = (i as u16) * 4 + 0;
        indices[i * 6 + 4] = (i as u16) * 4 + 2;
        indices[i * 6 + 5] = (i as u16) * 4 + 3;
        i += 1;
    }
    indices
};

const ATLAS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/FiraCode/FiraCode-Regular.json"
));

const ATLAS_TEXTURE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/FiraCode/FiraCode-Regular.png"
));

const SHADER_CODE: ShaderModuleDescriptor<'_> = include_shader!("Text shader", "text.wgsl");

pub struct TextRenderer {
    _shader: ShaderModule,
    atlas: FontAtlas,
    _atlas_texture: Texture,
    _atlas_view: TextureView,
    _sampler: Sampler,
    _bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
    vertex_buffer: StaticBuffer<Vertex>,
    index_buffer: StaticBuffer<u16>,
    _pipeline_layout: PipelineLayout,
    pipeline: RenderPipeline,
    vertices: Vec<Vertex>,
    resolution: Vec2f,
}

impl TextRenderer {
    pub fn create(wgpu_state: &WgpuState, width: u32, height: u32) -> Self {
        let shader = wgpu_state.device.create_shader_module(SHADER_CODE);

        let atlas = FontAtlas::load(ATLAS).unwrap();

        let atlas_texture_reader = std::io::Cursor::new(ATLAS_TEXTURE);
        let atlas_texture = wgpu_state.load_image(atlas_texture_reader, Some("Text atlas"), false);
        let atlas_view = atlas_texture.create_view(&TextureViewDescriptor::default());

        let sampler = wgpu_state.device.create_sampler(&SamplerDescriptor {
            label: Some("Text sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..Default::default()
        });

        let vertex_buffer = StaticBuffer::create(
            &wgpu_state.device,
            Some("Text vertices"),
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            MAX_VERTEX_COUNT,
        );

        let index_buffer = StaticBuffer::create_init(
            &wgpu_state.device,
            Some("Text indices"),
            BufferUsages::INDEX,
            &INDICES,
        );

        let bind_group_layout =
            wgpu_state
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                sample_type: TextureSampleType::Float { filterable: true },
                                view_dimension: TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Sampler(SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let bind_group = wgpu_state.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&atlas_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        let (pipeline_layout, pipeline) = wgpu_state.create_pipeline(
            "Text pipeline",
            &shader,
            &bind_group_layout,
            &[VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
                step_mode: VertexStepMode::Vertex,
                attributes: &vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Unorm8x4, 3 => Float32],
            }],
            Some(BlendState::ALPHA_BLENDING),
        );

        Self {
            _shader: shader,
            atlas,
            _atlas_texture: atlas_texture,
            _atlas_view: atlas_view,
            _sampler: sampler,
            _bind_group_layout: bind_group_layout,
            bind_group,
            vertex_buffer,
            index_buffer,
            _pipeline_layout: pipeline_layout,
            pipeline,
            vertices: Vec::with_capacity(MAX_VERTEX_COUNT),
            resolution: Vec2f::new(width as f32, height as f32),
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.resolution = Vec2f::new(new_width as f32, new_height as f32);
    }

    #[inline]
    fn transform_position(&self, pos: Vec2f, font_size: f32, offset: Vec2f) -> Vec2f {
        let world_pos = pos * font_size + offset;
        let clip_pos = ((world_pos / self.resolution) - 0.5) * Vec2f::new(2.0, -2.0);
        clip_pos
    }

    fn draw_batch(
        &mut self,
        wgpu_state: &WgpuState,
        render_target: &TextureView,
        encoder: &mut CommandEncoder,
    ) {
        self.vertex_buffer.write(&wgpu_state.queue, &self.vertices);

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Text pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: render_target,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice());
        pass.set_index_buffer(self.index_buffer.slice(), IndexFormat::Uint16);

        let index_count = ((self.vertices.len() / 4) * 6) as u32;
        pass.draw_indexed(0..index_count, 0, 0..1);

        self.vertices.clear();
    }

    pub fn draw_text(
        &mut self,
        wgpu_state: &WgpuState,
        render_target: &TextureView,
        encoder: &mut CommandEncoder,
        text: &str,
        position: Vec2f,
        font_size: f32, // in pixels
        color: [u8; 4],
    ) {
        let px_range = self.atlas.get_distance_range(font_size);

        let mut rel_x = 0.0;
        let mut prev: Option<char> = None;
        for c in text.chars() {
            if let Some(glyph) = self.atlas.get_glyph(c) {
                let kerning = self.atlas.get_kerning(prev, c);

                if let Some(sprite) = &glyph.sprite {
                    let top = sprite.bounds.top - self.atlas.ascender;
                    let bottom = sprite.bounds.bottom - self.atlas.ascender;
                    let left = rel_x + sprite.bounds.left + kerning;
                    let right = rel_x + sprite.bounds.right + kerning;

                    self.vertices.push(Vertex {
                        position: self.transform_position(
                            Vec2f::new(left, top),
                            font_size,
                            position,
                        ),
                        uv: Vec2f::new(sprite.uv_bounds.left, sprite.uv_bounds.top),
                        color,
                        px_range,
                    });
                    self.vertices.push(Vertex {
                        position: self.transform_position(
                            Vec2f::new(right, top),
                            font_size,
                            position,
                        ),
                        uv: Vec2f::new(sprite.uv_bounds.right, sprite.uv_bounds.top),
                        color,
                        px_range,
                    });
                    self.vertices.push(Vertex {
                        position: self.transform_position(
                            Vec2f::new(right, bottom),
                            font_size,
                            position,
                        ),
                        uv: Vec2f::new(sprite.uv_bounds.right, sprite.uv_bounds.bottom),
                        color,
                        px_range,
                    });
                    self.vertices.push(Vertex {
                        position: self.transform_position(
                            Vec2f::new(left, bottom),
                            font_size,
                            position,
                        ),
                        uv: Vec2f::new(sprite.uv_bounds.left, sprite.uv_bounds.bottom),
                        color,
                        px_range,
                    });
                }

                rel_x += glyph.x_advance + kerning;
                prev = Some(c);

                if self.vertices.len() >= MAX_VERTEX_COUNT {
                    self.draw_batch(wgpu_state, render_target, encoder);
                }
            }
        }
    }

    pub fn end_draw(
        &mut self,
        wgpu_state: &WgpuState,
        render_target: &TextureView,
        encoder: &mut CommandEncoder,
    ) {
        if !self.vertices.is_empty() {
            self.draw_batch(wgpu_state, render_target, encoder);
        }
    }
}
