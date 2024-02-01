pub mod config;
pub mod options;
pub mod surface;

use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin_winit::DisplayBuilder;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::{
    dpi::LogicalPosition,
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder},
};

use self::options::WindowOptions;

pub struct Window {
    pub window: WinitWindow,
    pub handle: RawWindowHandle,
    pub gl_config: glutin::config::Config,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>, options: WindowOptions) -> Self {
        let window_builder = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(options.size.0, options.size.1))
            .with_blur(options.blur)
            .with_title(options.title)
            .with_active(options.active)
            .with_visible(options.visible)
            .with_transparent(options.transparent)
            .with_blur(options.blur)
            .with_maximized(options.maximized)
            .with_resizable(options.resizable)
            .with_decorations(options.decorations)
            .with_window_icon(options.window_icon)
            .with_position(LogicalPosition::new(options.position.0, options.position.1));

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
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
        let window = window.expect("Could not create a window.");
        let handle = window.raw_window_handle();

        Self {
            window,
            handle,
            gl_config,
        }
    }

    pub fn window(&mut self) -> &mut WinitWindow {
        &mut self.window
    }

    pub fn handle(&mut self) -> &mut RawWindowHandle {
        &mut self.handle
    }

    pub fn gl_config(&mut self) -> &mut Config {
        &mut self.gl_config
    }
}
