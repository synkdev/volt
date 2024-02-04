use crate::ui::{Color::Hex, Element};

#[derive(Copy, Debug, Clone)]
pub struct Div {
    pub children: Vec<Box<dyn Element>>,
    pub z_index: usize,
    pub position: (f32, f32),
    pub fill: skia::Color,
    pub radius: f32,
    pub border_width: f32,
    pub border_color: skia::Color,
    pub size: (f32, f32),
    pub on_click: fn(&mut Self),
    pub on_click_release: fn(&mut Self),
    pub on_hover_enter: fn(&mut Self),
    pub on_hover_leave: fn(&mut Self),
    pub dirty: bool,
    pub hovered: bool,
    pub clicked: bool,
}
