use super::{element::Element, Color};

pub trait Widget: Element {
    fn set_fill(&mut self, fill: Color);
}
