#[derive(Debug)]
enum DeviceMessage {
    ButtonPressed,
    ButtonReleased,
    LedStatus { on: bool },
}

fn encode_message(message: &DeviceMessage) -> String {
    match message {
        DeviceMessage::ButtonPressed => "button:pressed".to_string(),
        DeviceMessage::ButtonReleased => "button:released".to_string(),
        DeviceMessage::LedStatus { on } => format!("led:{on}"),
    }
}

fn main() {
    let pressed = DeviceMessage::ButtonPressed;
    let released = DeviceMessage::ButtonReleased;
    let led = DeviceMessage::LedStatus { on: true };

    println!("{}", encode_message(&pressed));
    println!("{}", encode_message(&released));
    println!("{}", encode_message(&led));
}
