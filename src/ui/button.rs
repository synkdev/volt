use crate::ui::Component;
use skia_safe::{Canvas, Color, Paint, PaintStyle, RRect, Rect, Surface, Typeface};

pub struct Button {
    pub text: String,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: crate::ui::Color,
    pub radius: u32,
    pub border_width: u32,
    pub border_color: crate::ui::Color,
    pub text_color: crate::ui::Color,
}

impl Component for Button {
    fn render(&self, canvas: &skia_safe::canvas::Canvas) {
        let rect = Rect::from_xywh(self.position.0, self.position.1, self.size.0, self.size.1);

        // paint

        canvas.draw_round_rect(rect, 20.0, 20.0, paint);
    }
}
