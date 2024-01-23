use crate::ui::Component;

pub struct Button {
    pub text: String,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: skia_safe::Color,
    pub radius: u32,
    pub border_width: u32,
    pub border_color: skia_safe::Color,
    pub text_color: skia_safe::Color,
}

impl Component for Button {
    fn render(&self, canvas: &skia_safe::canvas::Canvas) {
        let rect =
            skia_safe::Rect::from_xywh(self.position.0, self.position.1, self.size.0, self.size.1);

        // paint
        let mut paint = skia_safe::Paint::default();
        paint.set_color(self.color);
        paint.set_style(skia_safe::PaintStyle::Fill);

        canvas.draw_round_rect(rect, 20.0, 20.0, &paint);
    }
}
