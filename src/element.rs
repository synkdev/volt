use taffy::NodeId;
use vello::Scene;

pub trait Element {
    fn render(&mut self, scene: &mut Scene);
    fn taffy_id(&self) -> NodeId;
}
