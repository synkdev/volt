use std::collections::HashMap;

use crate::element::Element;

pub struct Node<Element: Element> {
	element: Element,
	id: usize,
	children: Option<Vec<Node<Element>>>,
}
