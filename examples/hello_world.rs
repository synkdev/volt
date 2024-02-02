use volt::{
    ui::{button::ButtonBuilder, Color, Component},
    Volt,
};

fn main() {
    Volt::new().with_id("hello_world").run(|cx| {
        cx.add(
            "werks_btn",
            ButtonBuilder::new()
                .on_click(|_| println!("it werks"))
                .into(),
        );
        cx.add(
            "hello_btn",
            ButtonBuilder::new()
                .text("hello")
                .position(200.0, 200.0)
                .on_click(|_| println!("hello"))
                // .on_hover_enter(|btn| {
                //     println!("hovering");
                //     btn.fill = Color::Hex("#b4befe").into().unwrap();
                //     // println!("Before fill assignment: {:?}", btn.fill);
                //     // btn.fill = skia::Color::from_rgb(20, 30, 50);
                //     // println!("After fill assignment: {:?}", btn.fill);
                // })
                .on_hover_leave(|btn| {
                    println!("exitingfdd");
                    btn.fill = Color::Hex("#313244").into().unwrap();
                    // btn.fill = skia::Color::from_rgb(120, 230, 210);
                })
                .into(),
        );
        // somth
    });
}
