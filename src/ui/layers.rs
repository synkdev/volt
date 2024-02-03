use std::collections::HashMap;

use super::Element;

pub trait Layer: Element {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint);
    fn add(&mut self, element: dyn Element);
    fn set_z_index(&mut self, z_index: usize);
    fn get_bounds(&self) -> skia::Rect;
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, value: bool);
}

pub struct Layers {
    pub layers: HashMap<usize, Box<dyn Layer>>,
}
