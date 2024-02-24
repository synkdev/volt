use vello::peniko::Color as VelloColor;

/// Struct to represent hex, rgb and rgba colors. Implements a method to convert `self` into a `VelloColor` for rendering
#[derive(Clone)]
pub enum Color {
    /// Hex color. Example:
    /// ```rust,
    /// let hex_color = Color::Hex(0xf38ba8);
    /// ```
    Hex(u32),
    /// RGB color. Example:
    /// ```rust,
    /// let rgb_color = Color::Rgb((243, 139, 168));
    /// ```
    Rgb(u8, u8, u8),
    /// RGBA color. Example:
    /// ```rust,
    /// let rgba_color = Color::Rgba((243, 139, 168, 0.5));
    /// ```
    Rgba(u8, u8, u8, u8),
}

impl Color {
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

/// Enables any `u32` to be converted into a `Color`
impl Into<Color> for u32 {
    fn into(self) -> Color {
        Color::Hex(self)
    }
}

/// Converts a tuple with 3 fields into a RGB value
impl Into<Color> for (u8, u8, u8) {
    fn into(self) -> Color {
        Color::Rgb(self.0, self.1, self.2)
    }
}

/// Converts a tuple with 4 fields into a RGBA value
impl Into<Color> for (u8, u8, u8, u8) {
    fn into(self) -> Color {
        Color::Rgba(self.0, self.1, self.2, self.3)
    }
}
