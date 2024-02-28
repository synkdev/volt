use taffy::{NodeId, TaffyTree};
use crate::Element;

pub struct Context {
	pub elements: Vec<Box<dyn Element>>,
	pub root: NodeId,
	pub tree: TaffyTree,
}

impl Context {
	pub fn new() -> Self {
		Context {
			elements: vec![],
			root: NodeId::new(0),
			tree: TaffyTree::new(),
		}
	}
}
