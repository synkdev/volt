use crate::color::Color;
use std::sync::Arc;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::EventLoopWindowTarget,
    window::{Icon, Window, WindowBuilder},
};

#[derive(Clone)]
pub struct WindowOptions {
    pub title: &'static str,
    pub id: Option<&'static str>,
    pub size: (u32, u32),
    pub min_size: (u32, u32),
    pub max_size: (u32, u32),
    pub position: (i32, i32),
    pub resizable: bool,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub blur: bool,
    pub decorations: bool,
    pub window_icon: Option<Icon>,
    pub active: bool,
    pub background: Color,
}

impl Default for WindowOptions {
    fn default() -> Self {
        WindowOptions {
            title: "Volt App",
            id: Some("volt-app"),
            size: (1200, 700),
            min_size: (800, 400),
            max_size: (1600, 900),
            position: (0, 0),
            resizable: true,
            maximized: false,
            visible: true,
            transparent: false,
            blur: false,
            window_icon: None,
            active: true,
            decorations: false,
            background: Color::Hex("#1e1d2d"),
        }
    }
}

pub fn new(event_loop: &EventLoopWindowTarget<()>, options: WindowOptions) -> Arc<Window> {
    #[allow(unused_mut)]
    let mut window_builder = WindowBuilder::new()
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
        .with_position(LogicalPosition::new(options.position.0, options.position.1))
        .with_window_icon(options.window_icon);

    #[allow(dead_code)]
    #[cfg(feature = "wayland")]
    if let Some(app_id) = options.id {
        use winit::platform::wayland::WindowBuilderExtWayland as _;
        window_builder = window_builder.with_name(app_id, "");
    }
    #[allow(dead_code)]
    #[cfg(feature = "x11")]
    if let Some(app_id) = options.id {
        use winit::platform::x11::WindowBuilderExtX11 as _;
        window_builder = window_builder.with_name(app_id, "");
    }

    let window = window_builder
        .build(event_loop)
        .expect("Could not create a Winit window");

    Arc::new(window)
}
