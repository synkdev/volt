use downcast_rs::{impl_downcast, Downcast};
use skia::{Canvas, Paint};

pub trait Element: Downcast {
    /// Function to render the element;
    fn render(&self, canvas: Canvas, paint: Paint);

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
    fn is_clicked(&mut self) -> bool;
    /// Check if the element is hovered
    fn is_hovered(&mut self) -> bool;
    /// Check if the element is dirt. The element is only rendered if it is dirty
    fn is_dirty(&mut self) -> bool;

    /// Setters
    /// Set whether the element is clicked or not
    fn set_clicked(&mut self, value: bool);
    /// Set whether the element is hovered or not
    fn set_hovered(&mut self, value: bool);
    /// Set whether the element is dirty or not
    fn set_dirty(&mut self, value: bool);
}

impl_downcast!(Element);
