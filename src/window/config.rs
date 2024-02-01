use crate::window::Window;
use glutin::{
    context::{ContextApi, ContextAttributesBuilder, GlProfile, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    prelude::NotCurrentGlContext,
    surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};
use std::num::NonZeroU32;

pub struct GraphicsContext {
    pub gl_surface: GlutinSurface<WindowSurface>,
    pub gl_context: PossiblyCurrentContext,
}

impl GraphicsContext {
    pub fn new(window: &Window) -> Self {
        let context_attrs = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(Some(window.handle));
        let fallback_context_attrs = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::Gles(None))
            .build(Some(window.handle));

        let not_current_gl_context = unsafe {
            window
                .gl_config
                .display()
                .create_context(&window.gl_config, &context_attrs)
                .unwrap_or_else(|_| {
                    window
                        .gl_config
                        .display()
                        .create_context(&window.gl_config, &fallback_context_attrs)
                        .expect("Failed to create a context")
                })
        };

        let (width, height): (u32, u32) = window.window.inner_size().into();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window.handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let gl_surface = unsafe {
            window
                .gl_config
                .display()
                .create_window_surface(&window.gl_config, &attrs)
                .expect("Could not create a surface for the GL window!")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Could not set current context");

        GraphicsContext {
            gl_surface,
            gl_context,
        }
    }
}
