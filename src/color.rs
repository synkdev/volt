use vello::peniko::Color as VelloColor;

#[derive(Clone)]
pub enum Color {
    Hex(u32),
    Rgba(u8, u8, u8, u8),
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn default_hex() -> Self {
        Color::Hex(0x1e1d2d)
    }

    pub fn default_rgb() -> Self {
        Color::Rgb(30, 29, 45)
    }

    pub fn default_rgba() -> Self {
        Color::Rgba(30, 29, 45, 100)
    }
    pub fn into(self) -> VelloColor {
        match self {
            Color::Hex(hex) => {
                let r = ((hex >> 16) & 0xFF) as f64 / 255.0;
                let g = ((hex >> 8) & 0xFF) as f64 / 255.0;
                let b = (hex & 0xFF) as f64 / 255.0;
                VelloColor::rgb(r, g, b)
            }
            Color::Rgb(red, green, blue) => return VelloColor::rgb8(red, green, blue),
            Color::Rgba(red, green, blue, alpha) => {
                return VelloColor::rgba8(red, green, blue, alpha);
            }
        }
    }
}
