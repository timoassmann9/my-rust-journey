# Solution 01: Write Test Cases

Example test names:

```rust
#[test]
fn disconnected_begin_moves_to_connecting() {}

#[test]
fn connecting_success_moves_to_connected() {}

#[test]
fn connecting_fail_moves_to_disconnected() {}

#[test]
fn unknown_event_keeps_current_state() {}

#[test]
fn connected_unknown_event_stays_connected() {}
```
