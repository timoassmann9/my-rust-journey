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

Why this is the Rust way:

- `&mut String` means "borrow this buffer with permission to modify it"
- the caller still owns the string
- this pattern is common when reusing buffers in embedded and networking code
