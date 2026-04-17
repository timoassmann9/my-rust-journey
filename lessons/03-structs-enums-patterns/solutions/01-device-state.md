# Solution 01: Device State

```rust
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
```

Why this matters:

- the struct keeps related data together
- `summary(&self)` reads state without taking ownership
- this pattern scales much better than passing three booleans through every
  function
