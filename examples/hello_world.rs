use volt::run;
use volt::ui::Color;

fn main() {
    // run("Hello World!", 1200, 900).unwrap();
    let color = Color::Hex("1e1d2d".to_string()).into().unwrap();
    println!("{color:?}")
}
