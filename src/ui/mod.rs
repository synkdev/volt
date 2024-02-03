pub mod button;
pub mod color;
pub mod element;
pub mod layers;

pub use color::Color;
use downcast_rs::{impl_downcast, Downcast};

pub trait Component: Downcast {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint);
    fn on_click(&mut self);
    fn on_click_release(&mut self);
    fn on_hover_enter(&mut self);
    fn on_hover_leave(&mut self);
    fn is_hovered(&self) -> bool;
    fn is_dirty(&self) -> bool;
    fn is_visible(&self) -> bool;
    fn set_dirty(&mut self, value: bool);
    fn set_hovered(&mut self, value: bool);
    fn equals(&self, other: &dyn Component) -> bool;
    fn get_bounds(&self) -> skia::Rect;
}

impl_downcast!(Component);

impl PartialEq for Box<dyn Component> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other.as_ref())
    }
}
