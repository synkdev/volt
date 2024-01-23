pub mod button;

pub trait Component {
    fn render(&self, canvas: &skia_safe::canvas::Canvas);
}

pub enum Color {
    Hex(String),
    Rgba(u32, u32, u32, f64),
    Rgb(u32, u32, u32),
}

impl Color {
    pub fn default_hex() -> Self {
        Color::Hex("#1e1d2d".to_string())
    }

    pub fn default_rgb() -> Self {
        Color::Rgb(30, 29, 45)
    }

    pub fn default_rgba() -> Self {
        Color::Rgba(30, 29, 45, 1.0)
    }
}
