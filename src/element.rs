use taffy::NodeId;
use vello::Scene;

pub trait Element {
	fn render(&mut self, scene: &mut Scene);
	fn taffy_id(&self) -> NodeId;
	// fn on_hover(&mut self);
	// fn on_click(&mut self);
	// fn on_right_click(&mut self);
	// fn dirty() -> bool;
}
