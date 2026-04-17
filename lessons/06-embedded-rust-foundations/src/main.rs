#[derive(Debug, Clone, Copy)]
enum PinMode {
    Input,
    Output,
}

#[derive(Debug)]
struct PinConfig {
    pin: u8,
    mode: PinMode,
}

fn describe_pin(config: PinConfig) -> String {
    let mode = match config.mode {
        PinMode::Input => "input",
        PinMode::Output => "output",
    };

    format!("pin {} is {}", config.pin, mode)
}

fn main() {
    let led_pin = PinConfig {
        pin: 2,
        mode: PinMode::Output,
    };
    let button_pin = PinConfig {
        pin: 0,
        mode: PinMode::Input,
    };

    println!("{}", describe_pin(led_pin));
    println!("{}", describe_pin(button_pin));
}
