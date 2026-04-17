# Solution 02: Mutable Display Buffer

```rust
fn write_line(display: &mut String, line: &str) {
    display.push_str(line);
    display.push('\n');
}

fn clear_display(display: &mut String) {
    display.clear();
}
```
