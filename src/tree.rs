use std::collections::HashMap;

use crate::element::Element;

pub struct Tree {
	pub elements: HashMap<usize, Box<dyn Element>>,
}
