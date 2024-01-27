use volt::Context;

fn main() {
    let mut volt = Context::new().unwrap();
    volt.add(volt::ui::button::Button::new());
    volt.run().unwrap();
    println!("hello");
}
