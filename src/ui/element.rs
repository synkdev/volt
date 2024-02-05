use skia::{Canvas, Paint};
use winit::event::{ElementState, MouseButton};

pub trait Element {
    /// Function to render the element;
    fn render(&self, canvas: &Canvas, paint: &mut Paint);

    /// Events
    /// Mouse movement
    fn mouse_moved(&mut self, position: (f32, f32));
    /// Mouse click
    fn mouse_input(&mut self, state: ElementState, button: MouseButton, position: (f32, f32));

    /// Handlers
    /// Function to call when element is clicked
    fn on_click(&mut self);
    /// Function to call when the click is released
    fn on_click_release(&mut self);
    /// Function to call when the mouse enters the element
    fn on_hover_enter(&mut self);
    /// Function to call when the mouse leaves the element
    fn on_hover_leave(&mut self);

    /// Getters
    /// Check if the element is clicked
    fn is_clicked(&self) -> bool;
    /// Check if the element is hovered
    fn is_hovered(&self) -> bool;
    /// Check if the element is dirty. The element is only rendered if it is dirty
    fn is_dirty(&self) -> bool;
    /// Get the bounds of the element. Used for check if mouse is under an element
    fn get_bounds(&self) -> skia::Rect;
    /// Get Z-index of the element
    fn get_z_index(&self) -> usize;

    /// Setters
    /// Set whether the element is clicked or not
    fn set_clicked(&mut self, value: bool);
    /// Set whether the element is hovered or not
    fn set_hovered(&mut self, value: bool);
    /// Set whether the element is dirty or not
    fn set_dirty(&mut self, value: bool);
    /// Set the Z-index of the element
    fn set_z_index(&mut self, index: usize);
}
