use super::{Color, Element};

pub trait Widget: Element {
    fn set_fill(&mut self, fill: Color);
}
