# Solution 01: Architecture Draft

One reasonable shape:

```rust
struct AppState {
    button_pressed: bool,
    led_on: bool,
    wifi_connected: bool,
    display_lines: Vec<String>,
}

enum DeviceEvent {
    ButtonPressed,
    ButtonReleased,
    WifiConnected,
    WifiLost,
    HostCommand(HostCommand),
}

enum HostCommand {
    LedOn,
    LedOff,
    ClearDisplay,
    ShowText(String),
}
```

Module boundaries:

- hardware module: reads button, writes LED, writes display
- protocol module: encodes and decodes messages
- app module: owns state and decides behavior
