use crate::helpers::active_element;
use crate::Context;
use winit::event::MouseButton;

impl Context {
    pub fn process_click(&mut self, button: MouseButton, position: (f32, f32)) {
        if button == MouseButton::Left {
            match active_element(&mut self.components, position) {
                Some(component) => component.on_click(),
                None => return,
            }
        }
        self.render();
    }

    pub fn process_hover(&mut self, position: (f32, f32)) {
        match active_element(&mut self.components, position) {
            Some(component) => component.on_hover_enter(),
            None => {
                for component in &mut self.components.iter_mut() {
                    component.on_hover_leave();
                }
            }
        }
        // self.render();
    }
}
