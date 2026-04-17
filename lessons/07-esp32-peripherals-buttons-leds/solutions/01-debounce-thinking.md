# Solution 01: Debounce Thinking

Why bounce is a problem:

- one press may look like multiple presses
- if you toggle an LED on every detected press edge, the LED may flip several
  times from one real button action

Simple debounce rule:

- after accepting a press, ignore new press events for a short window such as
  20 to 50 milliseconds

Logic-first sketch:

```rust
fn accept_press(last_press_ms: Option<u64>, now_ms: u64, debounce_ms: u64) -> bool {
    match last_press_ms {
        Some(last) => now_ms.saturating_sub(last) >= debounce_ms,
        None => true,
    }
}
```
