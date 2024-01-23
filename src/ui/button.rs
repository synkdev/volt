use crate::ui::Component;

pub struct Button {
    pub text: String,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: skia_safe::Color,
    pub radius: f32,
    pub border_width: f32,
    pub border_color: skia_safe::Color,
    pub text_color: skia_safe::Color,
}

impl Component for Button {
    fn render(&self, canvas: &skia_safe::canvas::Canvas) {
        let rect =
            skia_safe::Rect::from_xywh(self.position.0, self.position.1, self.size.0, self.size.1);

        // Paint
        let mut paint = skia_safe::Paint::default();
        paint.set_color(self.color);
        paint.set_style(skia_safe::PaintStyle::Fill);

        // Draw button box
        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        paint.set_color(self.border_color);
        paint.set_style(skia_safe::PaintStyle::Stroke);
        paint.set_stroke_width(self.border_width);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);
    }
}
