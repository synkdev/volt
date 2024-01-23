pub mod button;

pub trait Component {
    fn render(&self, canvas: &skia_safe::canvas::Canvas);
}

pub enum Color {
    Hex(String),
    Rgba(u32, u32, u32, f64),
    Rgb(u32, u32, u32),
}
