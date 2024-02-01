use winit::window::Icon;

use crate::ui::Color;

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
