# Solution 01: Polling Loop Design

One good approach is to separate "read", "decide", and "write":

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

Why this is a useful first step:

- it models one loop iteration clearly
- it keeps behavior testable without real hardware
- the real hardware layer can later provide `button_pressed`
