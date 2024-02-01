use volt::{
    ui::{button::ButtonBuilder, Color},
    Volt,
};

fn main() {
    Volt::new().run(|cx| {
        cx.add(
            ButtonBuilder::new()
                .on_click(|_| println!("it werks"))
                .into(),
        );
        cx.add(
            ButtonBuilder::new()
                .text("hello")
                .position(200.0, 200.0)
                .on_click(|_| println!("hello"))
                .on_hover_enter(|btn| {
                    btn.fill = Color::Hex("#b4befe").into().unwrap();
                })
                .on_hover_leave(|btn| {
                    btn.fill = Color::Hex("#313244").into().unwrap();
                })
                .into(),
        );
        // somth
    });
}
