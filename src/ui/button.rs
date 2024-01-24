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
    pub fn set_text(&mut self, text: &'static str) {
        self.text = text;
    }
}
