pub(crate) mod helpers;
pub(crate) mod process_events;
pub mod ui;
pub(crate) mod window;

use crate::ui::Component;
use gl::types::*;
use gl_rs as gl;
use glutin::{config::GlConfig, prelude::GlSurface};
use skia::{gpu::gl::FramebufferInfo, Color, Paint};
use std::num::NonZeroU32;
use window::{config::GraphicsContext, surface::SkiaSurface, Window};
use winit::{
    event::{Event, KeyEvent, Modifiers, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
};

// Re-exports
pub use skia::font_style;

pub struct Volt {
    app: Context,
}

pub struct Context {
    modifiers: Modifiers,
    surface: SkiaSurface,
    window: Window,
    gr_context: GraphicsContext,
    event_loop: Option<EventLoop<()>>,
    clear: bool,
    paint: Paint,
    pub components: Vec<Box<dyn ui::Component>>,
}

impl Volt {
    pub fn new() -> Self {
        Volt {
            app: Context::new().unwrap(),
        }
    }

    pub fn run<F>(mut self, mut callback: F)
    where
        F: FnMut(&mut Context),
    {
        callback(&mut self.app);
        self.app.run().unwrap();
    }
}

impl Context {
    pub fn new() -> anyhow::Result<Self> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let window = Window::new(&event_loop);
        let gr_context = GraphicsContext::new(&window);
        let surface = SkiaSurface::new(&window);

        let modifiers = Modifiers::default();
        let paint = Paint::default();

        Ok(Context {
            event_loop: Some(event_loop),
            window,
            gr_context,
            surface,
            modifiers,
            clear: true,
            paint,
            components: Vec::new(),
        })
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = self.event_loop.take().unwrap();
        let mut cursor_pos = (0.0_f32, 0.0_f32);

        event_loop.run(move |event, window_target| {
            self.handle_events(event, window_target, &mut cursor_pos)
        })?;
        Ok(())
    }

    pub fn handle_events(
        &mut self,
        main_event: Event<()>,
        window_target: &EventLoopWindowTarget<()>,
        cursor_pos: &mut (f32, f32),
    ) {
        match main_event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    *cursor_pos = (position.x as f32, position.y as f32);
                    self.process_hover(*cursor_pos);
                }
                WindowEvent::CloseRequested => {
                    window_target.exit();
                    return;
                }
                WindowEvent::ModifiersChanged(new_modifiers) => self.modifiers = new_modifiers,
                WindowEvent::MouseInput { state, button, .. } => {
                    if state.is_pressed() {
                        self.process_click(button, *cursor_pos)
                    }
                }
                WindowEvent::KeyboardInput {
                    event: KeyEvent { logical_key, .. },
                    ..
                } => {
                    if self.modifiers.state().super_key() && logical_key == "q" {
                        window_target.exit();
                    }
                    self.window.window.request_redraw();
                }
                WindowEvent::Resized(physical_size) => {
                    let fb_info = {
                        let mut fboid: GLint = 0;
                        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

                        FramebufferInfo {
                            fboid: fboid.try_into().unwrap(),
                            format: skia::gpu::gl::Format::RGBA8.into(),
                            ..Default::default()
                        }
                    };
                    self.surface.surface = crate::window::surface::SkiaSurface::create_surface(
                        &self.window.window,
                        fb_info,
                        &mut self.surface.gr_context,
                        self.window.gl_config.num_samples() as usize,
                        self.window.gl_config.stencil_size() as usize,
                    );
                    let (width, height): (u32, u32) = physical_size.into();

                    self.gr_context.gl_surface.resize(
                        &self.gr_context.gl_context,
                        NonZeroU32::new(width.max(1)).unwrap(),
                        NonZeroU32::new(height.max(1)).unwrap(),
                    );
                    self.render();
                }
                WindowEvent::RedrawRequested => {
                    self.render();
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn render(&mut self) {
        self.start_render();
        self.finish_render();
    }

    pub fn start_render(&mut self) {
        println!("called mf");
        let canvas = self.surface.surface.canvas();
        // if self.clear {
        canvas.clear(Color::from_rgb(30, 29, 45));
        // canvas.save();
        // self.clear = false;
        // }
        // canvas.restore();
        self.draw();
    }

    pub fn draw(&mut self) {
        let canvas = self.surface.surface.canvas();

        'elements: for component in self.components.iter_mut() {
            if !component.is_visible() {
                continue 'elements;
            } else if component.is_dirty() {
                component.render(canvas, &mut self.paint);
                canvas.save();
                // component.was_drawn();
            } else {
                canvas.restore();
            }
        }
    }

    pub fn finish_render(&mut self) {
        self.gr_context
            .gl_surface
            .swap_buffers(&self.gr_context.gl_context)
            .unwrap();
        self.surface.gr_context.flush_and_submit();
    }

    pub fn add(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
        self.draw();
    }
}
