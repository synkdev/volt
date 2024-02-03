use std::collections::HashMap;

use crate::ui::Element;
use skia::Contains;

pub(crate) fn active_element(
    components: &mut HashMap<String, Box<dyn Element>>,
    position: (f32, f32),
) -> Option<(String, &mut Box<dyn Element>)> {
    for (id, component) in components.iter_mut() {
        let rect = component.get_bounds();
        if rect.contains(skia::Point::from(position)) {
            return Some((id.to_owned(), component));
        }
    }
    None
}

#[macro_export]
macro_rules! compare_fields {
    ($left:expr, $right:expr; $($field:ident),*) => {
        {
            $(
                if $left.$field != $right.$field {
                    return false;
                }
            )*
            true
        }
    };
}
pub(crate) use compare_fields;
