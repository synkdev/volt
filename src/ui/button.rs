use crate::ui::Component;
use skia_safe::{Canvas, Color, Paint, PaintStyle, Rect, Surface, Typeface};

pub struct Button {
    pub text: String,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub color: crate::ui::Color,
    pub radius: u32,
    pub border_width: u32,
    pub border_color: crate::ui::Color,
    pub text_color: crate::ui::Color,
}

impl Component for Button {
    fn render(&self, canvas: &skia_safe::canvas::Canvas) {}
}

pub fn button(canvas: &skia_safe::canvas::Canvas, center: (i32, i32), radius: i32) {}
