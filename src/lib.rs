pub(crate) mod helpers;
pub(crate) mod process_events;
pub mod ui;

use crate::ui::Component;
use gl::types::*;
use gl_rs as gl;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    prelude::{GlSurface, NotCurrentGlContext},
    surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use skia::{
    gpu::{self, backend_render_targets, gl::FramebufferInfo, SurfaceOrigin},
    Color, ColorType, Surface,
};
use std::{ffi::CString, num::NonZeroU32};
use winit::{
    dpi::LogicalSize,
    event::{Event, KeyEvent, Modifiers, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

// Re-exports
pub use skia::font_style;

pub struct Volt {
    app: Context,
}

pub struct Context {
    modifiers: Modifiers,
    paint: skia::Paint,
    surface: Surface,
    gl_surface: GlutinSurface<WindowSurface>,
    gr_context: skia::gpu::DirectContext,
    gl_context: PossiblyCurrentContext,
    gl_config: glutin::config::Config,
    window: Window,
    event_loop: Option<EventLoop<()>>,
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
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let winit_window_builder =
            WindowBuilder::new().with_inner_size(LogicalSize::new(1200, 700));
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);
        let display_builder = DisplayBuilder::new().with_window_builder(Some(winit_window_builder));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.expect("Couldn't create a window.");
        let raw_window_handle = window.raw_window_handle();

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_config
                        .display()
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        };

        let (width, height): (u32, u32) = window.inner_size().into();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .expect("Could not create gl window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Could not make GL context current when setting up skia renderer");

        gl::load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });
        let interface = skia::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        let mut gr_context = skia::gpu::DirectContext::new_gl(Some(interface), None)
            .expect("Could not create direct context");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };
        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;

        let surface =
            Self::create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size);

        let modifiers = Modifiers::default();
        let paint = skia::Paint::default();

        Ok(Context {
            surface,
            gl_surface,
            gl_context,
            gr_context,
            window,
            modifiers,
            paint,
            event_loop: Some(event_loop),
            components: Vec::new(),
            gl_config,
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
                    self.window.request_redraw();
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
                    self.surface = Self::create_surface(
                        &self.window,
                        fb_info,
                        &mut self.gr_context,
                        self.gl_config.num_samples() as usize,
                        self.gl_config.stencil_size() as usize,
                    );
                    let (width, height): (u32, u32) = physical_size.into();

                    self.gl_surface.resize(
                        &self.gl_context,
                        NonZeroU32::new(width.max(1)).unwrap(),
                        NonZeroU32::new(height.max(1)).unwrap(),
                    );
                }
                WindowEvent::RedrawRequested => self.draw(),
                _ => (),
            },
            _ => (),
        }
    }

    pub fn draw(&mut self) {
        let canvas = self.surface.canvas();
        canvas.clear(Color::from_rgb(30, 29, 45));
        for component in &self.components {
            component.render(canvas, &mut self.paint)
        }
        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }

    pub fn add(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
        self.draw();
    }

    pub fn create_surface(
        window: &Window,
        fb_info: FramebufferInfo,
        gr_context: &mut skia::gpu::DirectContext,
        num_samples: usize,
        stencil_size: usize,
    ) -> Surface {
        let size = window.inner_size();
        let size = (
            size.width.try_into().expect("Could not convert width"),
            size.height.try_into().expect("Could not convert height"),
        );
        let backend_render_target =
            backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

        gpu::surfaces::wrap_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Could not create skia surface")
    }
}
