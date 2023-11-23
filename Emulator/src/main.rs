#[macro_use]
extern crate static_assertions;

mod cpu;
mod display;
mod memory;
mod system;

type HashMap<K, V> = ahash::AHashMap<K, V>;

trait Ashr<Rhs = Self> {
    type Output;

    fn ashr(self, rhs: Rhs) -> Self::Output;
}

impl Ashr for u32 {
    type Output = Self;

    #[inline]
    fn ashr(self, rhs: Self) -> Self::Output {
        ((self as i32) >> rhs) as u32
    }
}

macro_rules! shuffle_bits {
    ($input:ident { [$src_end:literal : $src_start:literal] => [$dst_end:literal : $dst_start:literal] $(,)? }) => {{
        const_assert!($src_start >= 0);
        const_assert!($dst_start >= 0);
        const_assert!($src_end >= $src_start);
        const_assert!($dst_end >= $dst_start);
        const_assert_eq!($src_end - $src_start, $dst_end - $dst_start);

        let mask = !((!0) << ($src_end - $src_start + 1));
        (($input >> $src_start) & mask) << $dst_start
    }};
    ($input:ident { [$src:literal] => [$dst:literal] $(,)? }) => {{
        const_assert!($src >= 0);
        const_assert!($dst >= 0);

        (($input >> $src) & 0x1) << $dst
    }};
    ($input:ident { sign [$src:literal] => [$dst:literal] $(,)? }) => {{
        const_assert!($src >= 0);
        const_assert!($dst >= 0);

        let bit = ($input >> $src) & 0x1;
        let sign = (!bit).wrapping_add(1);
        sign << $dst
    }};
    ($input:ident { [$src_end:literal : $src_start:literal] => [$dst_end:literal : $dst_start:literal], $($t:tt)+ }) => {
        $crate::shuffle_bits!($input { [$src_end : $src_start] => [$dst_end : $dst_start] })
        | $crate::shuffle_bits!($input { $($t)+ })
    };
    ($input:ident { [$src:literal] => [$dst:literal], $($t:tt)+ }) => {
        $crate::shuffle_bits!($input { [$src] => [$dst] })
        | $crate::shuffle_bits!($input { $($t)+ })
    };
    ($input:ident { sign [$src:literal] => [$dst:literal], $($t:tt)+ }) => {
        $crate::shuffle_bits!($input { sign [$src] => [$dst] })
        | $crate::shuffle_bits!($input { $($t)+ })
    };
}

use shuffle_bits;

fn main() {
    use std::sync::atomic::{self, AtomicBool};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use system::EnvAction;
    use winit::dpi::PhysicalSize;
    use winit::event::{ElementState, Event, ModifiersState, VirtualKeyCode, WindowEvent};
    use winit::event_loop::EventLoop;
    use winit::window::WindowBuilder;

    const INITIAL_WINDOW_WIDTH: u32 = 800;
    const INITIAL_WINDOW_HEIGHT: u32 = 600;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Art32 Emu")
        .with_inner_size(PhysicalSize {
            width: INITIAL_WINDOW_WIDTH,
            height: INITIAL_WINDOW_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    let mut wgpu_state = display::WgpuState::create(&window);
    let vga = display::Vga::new(&wgpu_state);
    let mut text_renderer = display::TextRenderer::create(
        &wgpu_state,
        window.inner_size().width,
        window.inner_size().height,
    );

    let run = Arc::new(AtomicBool::new(false));
    let exit = Arc::new(AtomicBool::new(false));
    let art32 = Arc::new(Mutex::new(system::Art32::new()));

    let run_clone = Arc::clone(&run);
    let exit_clone = Arc::clone(&exit);
    let art32_clone = Arc::clone(&art32);

    let mut thread_handle = Some(thread::spawn(move || {
        let run = run_clone;
        let exit = exit_clone;
        let art32 = art32_clone;

        const INNER_ITER_COUNT: u32 = 10000;
        'outer: loop {
            if exit.load(atomic::Ordering::Acquire) {
                break 'outer;
            }

            if run.load(atomic::Ordering::Acquire) {
                let mut art32 = art32.lock().unwrap();

                'inner: for _ in 0..INNER_ITER_COUNT {
                    match art32.step() {
                        Some(EnvAction::Break) => {
                            run.store(false, atomic::Ordering::Release);
                            break 'inner;
                        }
                        Some(EnvAction::Reset) => {
                            println!("system reset requested");
                            art32.reset();
                        }
                        Some(EnvAction::Error) => {
                            panic!("system caused an error");
                        }
                        None => (),
                    }
                }
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }));

    let mut keyboard_modifiers = ModifiersState::empty();
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } if window_id == window.id() => {
                exit.store(true, atomic::Ordering::Release);
                thread_handle.take().unwrap().join().unwrap();
                control_flow.set_exit();
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(new_inner_size),
            } if window_id == window.id() => {
                wgpu_state.resize(new_inner_size.width, new_inner_size.height);
                text_renderer.resize(new_inner_size.width, new_inner_size.height);
                window.request_redraw();
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
            } if window_id == window.id() => {
                wgpu_state.resize(new_inner_size.width, new_inner_size.height);
                text_renderer.resize(new_inner_size.width, new_inner_size.height);
                window.request_redraw();
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::ModifiersChanged(new_modifiers),
            } if window_id == window.id() => {
                keyboard_modifiers = new_modifiers;
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::KeyboardInput { input, .. },
            } if window_id == window.id() => {
                if (input.state == ElementState::Pressed)
                    && keyboard_modifiers.contains(ModifiersState::CTRL)
                {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Space) => {
                            run.fetch_xor(true, atomic::Ordering::Release);
                        }
                        Some(VirtualKeyCode::R) => {
                            let mut art32 = art32.lock().unwrap();
                            art32.reset();
                        }
                        Some(VirtualKeyCode::C) => {
                            if !run.load(atomic::Ordering::Acquire) {
                                let mut art32 = art32.lock().unwrap();
                                match art32.step() {
                                    Some(EnvAction::Break) => {
                                        // already in single step mode
                                    }
                                    Some(EnvAction::Reset) => {
                                        println!("system reset requested");
                                        art32.reset();
                                    }
                                    Some(EnvAction::Error) => {
                                        panic!("system caused an error");
                                    }
                                    None => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                use wgpu::*;

                match wgpu_state.get_back_buffer() {
                    Ok(back_buffer) => {
                        let back_buffer_view = back_buffer
                            .texture
                            .create_view(&TextureViewDescriptor::default());

                        let mut encoder = wgpu_state.create_encoder();

                        vga.draw(&mut encoder, &back_buffer_view);

                        {
                            let art32 = art32.lock().unwrap();
                            art32.draw_debug_info(
                                &wgpu_state,
                                &back_buffer_view,
                                &mut encoder,
                                &mut text_renderer,
                            );
                        }

                        text_renderer.end_draw(&wgpu_state, &back_buffer_view, &mut encoder);

                        wgpu_state.submit_encoder(encoder);
                        back_buffer.present();

                        window.request_redraw();
                    }
                    Err(SurfaceError::Outdated) => {}
                    Err(err) => panic!("{err}"),
                }
            }
            _ => {}
        }
    });
}
