use volt::{
    ui::{button::ButtonBuilder, Color},
    Volt,
};

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
                // .on_hover_enter(|btn| btn.fill = Color::Hex("#b4befe").into().unwrap())
                .into(),
        );
        // somth
    });
}
