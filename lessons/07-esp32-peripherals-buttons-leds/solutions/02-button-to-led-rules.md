# Solution 02: Button to LED Rules

```rust
#[derive(Debug)]
struct Controller {
    led_on: bool,
    press_count: u32,
}

impl Controller {
    fn handle_button(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => {
                self.led_on = !self.led_on;
                self.press_count += 1;
            }
            ButtonState::Released => {}
        }
    }
}
```

Why this is better than ad-hoc flags:

- the controller owns its behavior state
- pressed and released events are handled explicitly
