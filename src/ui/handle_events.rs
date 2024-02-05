use crate::ui::element::Element;
use skia::Contains;

pub(crate) enum MouseEventType {
    Entered,
    Exited,
    None,
}

pub(crate) fn get_active_element<'t>(
    elements: &mut Vec<Box<dyn Element>>,
    active_element: Option<&mut Box<dyn Element>>,
    position: (f32, f32),
) -> (Option<&'t mut Box<dyn Element>>, MouseEventType) {
    match active_element {
        Some(_) => return (None, MouseEventType::Exited),
        None => return (None, MouseEventType::None),
    }
}
