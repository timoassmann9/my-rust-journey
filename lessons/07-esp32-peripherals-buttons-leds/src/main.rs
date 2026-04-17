#[derive(Debug, Clone, Copy, PartialEq)]
enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug)]
struct Controller {
    led_on: bool,
}

impl Controller {
    fn handle_button(&mut self, state: ButtonState) {
        if state == ButtonState::Pressed {
            self.led_on = !self.led_on;
        }
    }
}

fn main() {
    let mut controller = Controller { led_on: false };
    controller.handle_button(ButtonState::Pressed);
    controller.handle_button(ButtonState::Released);
    println!("{:?}", controller.led_on);
}
