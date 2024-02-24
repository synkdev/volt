use vello::peniko::Color as VelloColor;

#[derive(Clone)]
pub enum Color {
    Hex(&'static str),
    Rgba(u8, u8, u8, u8),
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn default_hex() -> Self {
        Color::Hex("#1e1d2d")
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
                let hex = hex.strip_prefix("#").unwrap_or(hex);

                if hex.len() != 6 && hex.len() != 3 && hex.len() != 8 {
                    panic!("Invalid length for hex color. Must be 3, 6 or 8 characters long (excluding the # symbol if any)");
                } else if hex.len() == 3 {
                    let red =
                        u8::from_str_radix(&format!("{}{}", &hex[0..1], &hex[0..1]), 16).unwrap();
                    let green =
                        u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16).unwrap();
                    let blue =
                        u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16).unwrap();

                    return VelloColor::rgb8(red, green, blue);
                } else if hex.len() == 6 {
                    let red = u8::from_str_radix(&hex[0..2], 16).unwrap();
                    let green = u8::from_str_radix(&hex[2..4], 16).unwrap();
                    let blue = u8::from_str_radix(&hex[4..6], 16).unwrap();

                    return VelloColor::rgb8(red, green, blue);
                } else if hex.len() == 8 {
                    let red = u8::from_str_radix(&hex[0..2], 16).unwrap();
                    let green = u8::from_str_radix(&hex[2..4], 16).unwrap();
                    let blue = u8::from_str_radix(&hex[4..6], 16).unwrap();
                    let alpha = u8::from_str_radix(&hex[6..8], 16).unwrap();

                    return VelloColor::rgba8(red, green, blue, alpha);
                }

                VelloColor::BLACK
            }
            Color::Rgb(red, green, blue) => return VelloColor::rgb8(red, green, blue),
            Color::Rgba(red, green, blue, alpha) => {
                return VelloColor::rgba8(red, green, blue, alpha);
            }
        }
    }
}
