# Solution 01: Architecture Draft

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
