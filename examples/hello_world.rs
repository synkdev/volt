use volt::Volt;

fn main() {
    let mut volt = Volt::new("Hello World!", 1200, 900).unwrap();
    volt.add(volt::ui::button::Button::new());
    volt.run().unwrap();
}
