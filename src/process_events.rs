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
        self.draw();
    }

    pub fn process_hover_enter(&mut self, position: (f32, f32)) {
        match active_element(&mut self.components, position) {
            Some(component) => component.on_hover(),
            None => return,
        }
        self.draw();
    }
}
