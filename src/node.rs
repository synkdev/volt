use std::collections::HashMap;

use taffy::NodeId;

use crate::element::Element;

pub struct Node {
	pub element: Box<dyn Element>,
	pub id: NodeId,
}

impl Node {
	pub fn new(element: impl Element + 'static, id: usize) -> Self {
		Node {
			element: Box::new(element),
			id,
		}
	}
	pub fn add(&mut self, element: impl Element) {
		self.element.add(element);
	}
}
