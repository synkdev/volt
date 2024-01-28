use crate::ui::Component;

pub struct Button {
    pub text: &'static str,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: skia::Color,
    pub radius: f32,
    pub border_width: f32,
    pub border_color: skia::Color,
    pub text_color: skia::Color,
    pub font_size: f32,
    pub font_family: &'static str,
    pub font_weight: skia::font_style::Weight,
    pub font_style: skia::font_style::Slant,
    pub on_click: Box<dyn FnOnce(&mut Self)>,
}

pub struct ButtonBuilder {
    pub button: Box<Button>,
}

impl Component for Button {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint) {
        let rect =
            skia::Rect::from_xywh(self.position.0, self.position.1, self.size.0, self.size.1);

        paint.set_anti_alias(true);

        // Draw button box
        paint.set_color(self.color);
        paint.set_style(skia::PaintStyle::Fill);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        // Draw border
        paint.set_color(self.border_color);
        paint.set_style(skia::PaintStyle::Stroke);
        paint.set_stroke_width(self.border_width);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        // Draw text
        paint.set_color(self.text_color);
        paint.set_style(skia::PaintStyle::Fill);
        let font_style = skia::font_style::FontStyle::new(
            self.font_weight,
            skia::font_style::Width::NORMAL,
            self.font_style,
        );
        let font_family = skia::FontMgr::new()
            .match_family_style(self.font_family, font_style)
            .unwrap();
        let font = skia::Font::from_typeface(font_family, self.font_size);
        let text = skia::TextBlob::from_str(self.text, &font).unwrap();
        let text_bounds = font.measure_text(self.text, Some(&paint));
        let text_x = rect.center_x() - (text_bounds.1.width() / 2.0);
        let text_offset =
            ((font.metrics().1.descent - font.metrics().1.ascent) / 2.0) - font.metrics().1.descent;
        let text_y = rect.center_y() + text_offset;
        canvas.draw_text_blob(text, (text_x, text_y), &paint);
    }
}

impl Button {
    pub fn new() -> Self {
        Button {
            text: "Button",
            size: (200.0, 50.0),
            position: (0.0, 0.0),
            color: crate::ui::color::Color::Hex("#313244".to_string())
                .into()
                .unwrap(),
            radius: 10.0,
            font_size: 16.0,
            text_color: crate::ui::color::Color::Hex("#f38ba8".to_string())
                .into()
                .unwrap(),
            border_color: crate::ui::color::Color::Hex("#f38ba8".to_string())
                .into()
                .unwrap(),
            border_width: 2.0,
            font_style: crate::font_style::Slant::Upright,
            font_weight: crate::font_style::Weight::NORMAL,
            font_family: "JetBrains Mono",
            on_click: Box::new(|_| {}),
        }
    }
}

impl ButtonBuilder {
    pub fn new() -> Self {
        ButtonBuilder {
            button: Box::new(Button::new()),
        }
    }

    pub fn text(mut self, text: &'static str) -> Self {
        self.button.text = text;
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.button.position = (x, y);
        self
    }

    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        self.on_click = Box::new(callback);
        self
    }

    pub fn into(self) -> Box<Button> {
        self.button
    }
}
