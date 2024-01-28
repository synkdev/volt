use crate::ui::Component;
use skia::Contains;

pub(crate) fn active_element(
    components: &mut Vec<Box<dyn Component>>,
    position: (f32, f32),
) -> Option<&mut Box<dyn Component>> {
    for component in components.iter_mut() {
        let rect = component.get_bounds();
        println!("{rect:?}");
        println!("{position:?}");
        if rect.contains(skia::Point::from(position)) {
            return Some(component);
        }
    }
    None
}
