struct DeviceState {
    button_pressed: bool,
    led_on: bool,
    wifi_connected: bool,
}

impl DeviceState {
    fn summary(&self) -> String {
        format!(
            "button={}, led={}, wifi={}", 
            self.button_pressed, self.led_on, self.wifi_connected
        )
    }
}

enum ScreenCommand {
    Clear,
    WriteLine(String),
    SetBrightness(u8),
}

fn command_name(command: &ScreenCommand) -> &'static str {
    match command {
        ScreenCommand::Clear => "Clear",
        ScreenCommand::WriteLine(_) => "WriteLine",
        ScreenCommand::SetBrightness(_) => "SetBrightness",
    }
}

fn format_command(command: &ScreenCommand) -> String {
    match command {
        ScreenCommand::Clear => {
            String::from("Clear")
        },
        ScreenCommand::WriteLine(line) => {
            format!("Write line: {line}")
        },
        ScreenCommand::SetBrightness(brightness) => {
            format!("Set brightness: {brightness}")
        },
    }
}

fn main() {
    let device_state = DeviceState{
        button_pressed: false,
        led_on: true,
        wifi_connected: true,
    };
    let summary = device_state.summary();
    println!("{summary}\n");

    let command1 = ScreenCommand::Clear;
    let command2 = ScreenCommand::WriteLine(String::from("this is a new line"));
    let command3 = ScreenCommand::SetBrightness(16);

    let commands = vec![command1, command2, command3];
    for command in commands.iter() {
        println!("Command name: {}", command_name(command));
        println!("Command formatted: {}", format_command(command));
    }
}