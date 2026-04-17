# Solution 02: Design a Display Trait

```rust
trait Display {
    fn clear(&mut self);
    fn write_line(&mut self, text: &str);
    fn set_brightness(&mut self, level: u8);
}
```
