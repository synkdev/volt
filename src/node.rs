use std::collections::HashMap;

use crate::element::Element;

pub struct Node {
	element: Box<dyn Element>,
	id: usize,
}

impl Node {
	pub fn new(element: impl Element + 'static, id: usize) -> Self {
		Node {
			element: Box::new(element),
			id,
		}
	}
}
