use volt::{
    ui::{elements::button::Button, Color, Element},
    Volt,
};

fn main() {
    Volt::new().with_id("hello_world").run(|cx| {
        cx.root.add(Button::new().into());
    });
}
