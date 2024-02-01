use crate::window::Window;
use gl::types::*;
use gl_rs as gl;
use glutin::{
    config::GlConfig,
    display::{GetGlDisplay, GlDisplay},
};
use skia::{
    gpu::{
        self, backend_render_targets,
        gl::{Format, FramebufferInfo, Interface},
        DirectContext, SurfaceOrigin,
    },
    ColorType, Surface,
};
use std::ffi::CString;
use winit::window::Window as WinitWindow;

pub struct SkiaSurface {
    pub gr_context: skia::gpu::DirectContext,
    pub surface: Surface,
}

impl SkiaSurface {
    pub fn new(window: &Window) -> Self {
        gl::load_with(|s| {
            window
                .gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });

        let interface = Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            window
                .gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create Skia interface");

        let mut gr_context = DirectContext::new_gl(Some(interface), None)
            .expect("Could not create a Skia DirectContext");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let num_samples = window.gl_config.num_samples() as usize;
        let stencil_size = window.gl_config.stencil_size() as usize;

        let surface = Self::create_surface(
            &window.window,
            fb_info,
            &mut gr_context,
            num_samples,
            stencil_size,
        );

        SkiaSurface {
            gr_context,
            surface,
        }
    }

    pub fn create_surface(
        window: &WinitWindow,
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
