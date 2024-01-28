pub mod button;
pub mod color;

pub use color::Color;
use std::any::Any;

pub trait Component: Any {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint);
    fn on_click(&self);
    fn on_hover(&mut self);
    fn get_bounds(&self) -> skia::Rect;
}
