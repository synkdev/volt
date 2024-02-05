use crate::ui::element::Element;
use skia::Contains;

pub(crate) enum MouseEventType {
    Entered,
    Exited,
    None,
}

pub(crate) fn get_active_element<'t>(
    elements: &'t mut Vec<Box<dyn Element>>,
    active_element: Option<usize>,
    position: (f32, f32),
) -> (Option<usize>, MouseEventType) {
    for (index, element) in elements.iter_mut().enumerate() {
        let bounds = element.get_bounds();
        if bounds.contains(skia::Point::from(position)) {
            return match active_element {
                Some(active_index) if active_index == index => (Some(index), MouseEventType::None),
                _ => (Some(index), MouseEventType::Entered),
            };
        }
    }

    match active_element {
        Some(_) => (None, MouseEventType::Exited),
        None => (None, MouseEventType::None),
    }
}
