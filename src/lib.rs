pub(crate) mod context;
pub(crate) mod helpers;
pub(crate) mod process_events;
pub mod ui;
pub(crate) mod window;

use crate::context::Context;
use crate::window::options::WindowOptions;

// Re-exports
use image::GenericImageView;
pub use skia::font_style;
use winit::window::Icon;

pub struct Volt {
    options: WindowOptions,
}

impl Volt {
    pub fn new() -> Self {
        Volt {
            options: WindowOptions::default(),
        }
    }

    pub fn with_title(mut self, title: &'static str) -> Self {
        self.options.title = title;
        self
    }

    pub fn with_id(mut self, id: &'static str) -> Self {
        self.options.id = Some(id);
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.options.size = (width, height);
        self
    }

    pub fn with_min_size(mut self, width: u32, height: u32) -> Self {
        self.options.min_size = (width, height);
        self
    }

    pub fn with_max_size(mut self, width: u32, height: u32) -> Self {
        self.options.max_size = (width, height);
        self
    }

    pub fn with_position(mut self, x: u32, y: u32) -> Self {
        self.options.max_size = (x, y);
        self
    }

    pub fn with_resizable(mut self, enabled: bool) -> Self {
        self.options.resizable = enabled;
        self
    }

    pub fn with_maximized(mut self, enabled: bool) -> Self {
        self.options.maximized = enabled;
        self
    }

    pub fn with_visible(mut self, enabled: bool) -> Self {
        self.options.visible = enabled;
        self
    }

    pub fn with_transparent(mut self, enabled: bool) -> Self {
        self.options.transparent = enabled;
        self
    }

    pub fn with_blur(mut self, enabled: bool) -> Self {
        self.options.blur = enabled;
        self
    }

    pub fn with_decorations(mut self, enabled: bool) -> Self {
        self.options.decorations = enabled;
        self
    }

    pub fn with_icon(mut self, path: &'static str) -> Self {
        let image = image::open(path).unwrap();
        let rgba = image.to_rgba8().into_raw();
        let (width, height) = image.dimensions();

        self.options.window_icon = Some(Icon::from_rgba(rgba, width, height).unwrap());
        self
    }

    pub fn with_active(mut self, enabled: bool) -> Self {
        self.options.active = enabled;
        self
    }

    pub fn run<F>(self, mut callback: F)
    where
        F: FnMut(&mut Context),
    {
        let mut app = Context::new(self.options).expect("Could not create a Context!");

        callback(&mut app);
        app.run().expect("Could not run the app!");
    }
}
