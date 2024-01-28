use volt::{ui::button::ButtonBuilder, Volt};

fn main() {
    Volt::new().run(|cx| {
        cx.add(ButtonBuilder::new().into());
        cx.add(
            ButtonBuilder::new()
                .text("hello")
                .position(200.0, 200.0)
                .into(),
        );
        // somth
    });
}
