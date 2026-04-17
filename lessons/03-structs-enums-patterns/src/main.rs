#[derive(Debug)]
struct Led {
    pin: u8,
    is_on: bool,
}

impl Led {
    fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}

#[derive(Debug)]
enum ButtonEvent {
    Pressed,
    Released,
}

fn describe_event(event: ButtonEvent) -> &'static str {
    match event {
        ButtonEvent::Pressed => "button pressed",
        ButtonEvent::Released => "button released",
    }
}

fn main() {
    let mut led = Led {
        pin: 2,
        is_on: false,
    };
    led.toggle();
    println!("{led:?}");
    println!("led pin: {}", led.pin);

    println!("{}", describe_event(ButtonEvent::Pressed));
    println!("{}", describe_event(ButtonEvent::Released));
}
