# Solution 02: Design a Display Trait

One reasonable addition:

```rust
trait Display {
    fn clear(&mut self);
    fn write_line(&mut self, text: &str);
    fn set_brightness(&mut self, level: u8);
}
```

Why it belongs in the trait:

- brightness is part of display behavior, not application state

What the app should know:

- it can request a brightness level

What the app should not know:

- which I2C address is used
- how many bytes are sent
- which command sequence the hardware needs internally
