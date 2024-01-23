use crate::ui::Component;

pub struct Button {
    pub text: &'static str,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: skia_safe::Color,
    pub radius: f32,
    pub border_width: f32,
    pub border_color: skia_safe::Color,
    pub text_color: skia_safe::Color,
    pub font_size: f32,
    pub font_family: &'static str,
    pub font_weight: skia_safe::font_style::Weight,
    pub font_style: skia_safe::font_style::Slant,
}

impl Component for Button {
    fn render(&self, canvas: &skia_safe::canvas::Canvas) {
        let rect =
            skia_safe::Rect::from_xywh(self.position.0, self.position.1, self.size.0, self.size.1);

        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);

        // Draw button box
        paint.set_color(self.color);
        paint.set_style(skia_safe::PaintStyle::Fill);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        // Draw border
        paint.set_color(self.border_color);
        paint.set_style(skia_safe::PaintStyle::Stroke);
        paint.set_stroke_width(self.border_width);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        // Draw text
        paint.set_color(self.text_color);
        paint.set_style(skia_safe::PaintStyle::Fill);
        let font_style = skia_safe::font_style::FontStyle::new(
            self.font_weight,
            skia_safe::font_style::Width::NORMAL,
            self.font_style,
        );
        let font_family = skia_safe::FontMgr::new()
            .match_family_style(self.font_family, font_style)
            .unwrap();
        let font = skia_safe::Font::from_typeface(font_family, self.font_size);
        let text = skia_safe::TextBlob::from_str(self.text, &font).unwrap();
        canvas.draw_text_blob(text, (10.0, 10.0), &paint);
    }
}
