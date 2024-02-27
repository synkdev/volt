use std::collections::HashMap;

use crate::element::Element;

pub struct Node<E: Element> {
	element: E,
	id: usize,
	children: Option<Vec<Node<E>>>,
}

pub struct Tree {
	pub nodes: Vec<Node>,
}
