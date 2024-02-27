use std::collections::HashMap;

use crate::element::Element;

pub struct Node {
	element: dyn Element,
	id: usize,
	children: Option<Vec<Node>>,
}
