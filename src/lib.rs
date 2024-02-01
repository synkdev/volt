pub(crate) mod context;
pub(crate) mod helpers;
pub(crate) mod process_events;
pub mod ui;
pub(crate) mod window;

use crate::context::Context;

// Re-exports
pub use skia::font_style;

pub struct Volt {
    app: Context,
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
