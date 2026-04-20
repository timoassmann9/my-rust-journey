# Solution 01: Polling Loop Design

```rust
fn run_iteration(button_pressed: bool, led_on: &mut bool, display: &mut String) {
    if button_pressed {
        *led_on = !*led_on;
        display.push_str("button pressed\n");
    } else {
        display.push_str("idle\n");
    }
}
```
