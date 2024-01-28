pub mod button;
pub mod color;

pub use color::Color;

pub trait Component {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint);
}

pub trait Clickable {
    fn on_click(&self);
}
