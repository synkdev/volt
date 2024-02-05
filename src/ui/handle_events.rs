use crate::ui::element::Element;
use skia::Contains;

pub(crate) enum MouseEventType {
    Entered,
    Exited,
    None,
}

pub(crate) fn get_active_element<'t>(
    elements: &'t mut Vec<Box<dyn Element>>,
    active_element: Option<&mut Box<dyn Element>>,
    position: (f32, f32),
) -> (Option<&'t mut Box<dyn Element>>, MouseEventType) {
    for element in elements.iter_mut() {
        let bounds = element.get_bounds();
        if bounds.contains(skia::Point::from(position)) {
            return match active_element {
                Some(curr_element) if std::ptr::eq(curr_element.as_ref(), element.as_ref()) => {
                    (Some(element), MouseEventType::None)
                }
                _ => (Some(element), MouseEventType::Entered),
            };
        }
    }
    match active_element {
        Some(_) => return (None, MouseEventType::Exited),
        None => return (None, MouseEventType::None),
    }
}
