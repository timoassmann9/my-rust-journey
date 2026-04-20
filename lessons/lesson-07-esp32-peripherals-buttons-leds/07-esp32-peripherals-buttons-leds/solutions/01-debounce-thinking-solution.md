# Solution 01: Debounce Thinking

One simple debounce rule:

- after accepting a press, ignore any new press events for the next 20 to 50 ms

Code sketch:

```rust
fn accept_press(last_press_ms: Option<u64>, now_ms: u64, debounce_ms: u64) -> bool {
    match last_press_ms {
        Some(last) => now_ms.saturating_sub(last) >= debounce_ms,
        None => true,
    }
}
```
