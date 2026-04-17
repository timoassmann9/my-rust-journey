trait Display {
    fn clear(&mut self);
    fn write_line(&mut self, text: &str);
}

#[derive(Default, Debug)]
struct FakeDisplay {
    lines: Vec<String>,
}

impl Display for FakeDisplay {
    fn clear(&mut self) {
        self.lines.clear();
    }

    fn write_line(&mut self, text: &str) {
        self.lines.push(text.to_string());
    }
}

fn show_status(display: &mut dyn Display, wifi_ok: bool, led_on: bool) {
    display.clear();
    display.write_line(if wifi_ok { "wifi: ok" } else { "wifi: down" });
    display.write_line(if led_on { "led: on" } else { "led: off" });
}

fn main() {
    let mut display = FakeDisplay::default();
    show_status(&mut display, true, false);
    println!("{display:?}");
}
