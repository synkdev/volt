use std::collections::HashMap;

use super::Component;
use downcast_rs::{impl_downcast, Downcast};

pub trait Layer: Downcast {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint);
    fn add(&mut self, component: dyn Component);
    fn set_z_index(&mut self, z_index: usize);
    fn get_bounds(&self) -> skia::Rect;
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, value: bool);
}

impl_downcast!(Layer);

pub struct Layers {
    pub layers: HashMap<usize, Box<dyn Component>>,
}
