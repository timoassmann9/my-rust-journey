#[derive(Debug)]
struct AppState {
    button_pressed: bool,
    led_on: bool,
    wifi_connected: bool,
    last_message: Option<String>,
}

impl AppState {
    fn summary(&self) -> String {
        format!(
            "button={}, led={}, wifi={}, last_message={:?}",
            self.button_pressed, self.led_on, self.wifi_connected, self.last_message
        )
    }
}

fn main() {
    let state = AppState {
        button_pressed: false,
        led_on: false,
        wifi_connected: false,
        last_message: None,
    };

    println!("{}", state.summary());
}
