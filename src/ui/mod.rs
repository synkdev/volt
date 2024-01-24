pub mod button;
pub mod color;

pub use color::Color;

pub trait Component {
    fn render(&self, canvas: &skia_safe::canvas::Canvas);
}
