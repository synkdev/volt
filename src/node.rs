use std::collections::HashMap;

use crate::element::Element;

pub struct Node {
	element: Box<dyn Element>,
	id: usize,
}
