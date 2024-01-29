pub mod button;
pub mod color;

pub use color::Color;
use std::any::Any;

pub trait Component: Any {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint);
    fn on_click(&mut self);
    fn on_hover_enter(&mut self);
    fn on_hover_leave(&mut self);
    fn is_dirty(&self) -> bool;
    fn is_visible(&self) -> bool;
    fn was_drawn(&mut self);
    fn get_bounds(&self) -> skia::Rect;
}
