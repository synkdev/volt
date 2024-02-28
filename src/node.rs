use std::collections::HashMap;

use crate::element::Element;

pub struct Node {
	pub element: Box<dyn Element>,
	pub id: usize,
}

impl Node {
	pub fn new(element: impl Element + 'static, id: usize) -> Self {
		Node {
			element: Box::new(element),
			id,
		}
	}
}
