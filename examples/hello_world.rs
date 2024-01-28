use volt::{ui::button::ButtonBuilder, Volt};

fn main() {
    Volt::new().run(|cx| {
        cx.add(
            ButtonBuilder::new()
                .on_click(|| println!("it werks"))
                .into(),
        );
        cx.add(
            ButtonBuilder::new()
                .text("hello")
                .position(200.0, 200.0)
                .on_click(|| println!("hello"))
                .into(),
        );
        // somth
    });
}
