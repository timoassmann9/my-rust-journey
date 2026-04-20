# Solution 01: Add Screen Message

```rust
fn show_status(
    display: &mut dyn Display,
    wifi_ok: bool,
    led_on: bool,
    button_pressed: bool,
) {
    display.clear();
    display.write_line(if wifi_ok { "wifi: ok" } else { "wifi: down" });
    display.write_line(if led_on { "led: on" } else { "led: off" });
    display.write_line(if button_pressed {
        "button: pressed"
    } else {
        "button: released"
    });
}
```
