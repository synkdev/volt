use gl::types::*;
use gl_rs as gl;
use glutin::{config::GlConfig, prelude::GlSurface};
use skia::gpu::gl::FramebufferInfo;
use std::num::NonZeroU32;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoopWindowTarget,
};

use super::Context;
use crate::ui::Element;

impl Context {
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
                    self.root.mouse_moved(*cursor_pos);
                }
                WindowEvent::CloseRequested => {
                    window_target.exit();
                    return;
                }
                WindowEvent::ModifiersChanged(new_modifiers) => self.modifiers = new_modifiers,
                WindowEvent::MouseInput { state, button, .. } => {
                    self.root.mouse_input(state, button, *cursor_pos);
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
                    self.root.full_redraw = true;
                    self.root.set_dirty(true);
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
}
