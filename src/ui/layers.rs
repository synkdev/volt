use super::Element;

pub trait Layer: Element {
    fn add(&mut self, element: dyn Element);
    fn set_z_index(&mut self, z_index: usize);
}
