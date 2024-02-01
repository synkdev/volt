pub(crate) mod context;
pub(crate) mod helpers;
pub(crate) mod process_events;
pub mod ui;
pub(crate) mod window;

use crate::context::Context;
use crate::window::options::WindowOptions;

// Re-exports
pub use skia::font_style;

pub struct Volt {
    options: WindowOptions,
}

impl Volt {
    pub fn new() -> Self {
        Volt {
            options: WindowOptions::default(),
        }
    }

    pub fn run<F>(mut self, mut callback: F)
    where
        F: FnMut(&mut Context),
    {
        let mut app = Context::new(self.options).expect("Could not create a Context!");

        callback(&mut app);
        app.run().expect("Could not run the app!");
    }
}
