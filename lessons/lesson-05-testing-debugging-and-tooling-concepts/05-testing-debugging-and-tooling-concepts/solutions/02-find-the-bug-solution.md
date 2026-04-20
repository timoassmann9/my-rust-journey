# Solution 02: Find the Bug

```rust
fn next_state(current: WifiState, event: &str) -> WifiState {
    match (current, event) {
        (WifiState::Disconnected, "begin") => WifiState::Connecting,
        (WifiState::Connecting, "success") => WifiState::Connected,
        (WifiState::Connecting, "fail") => WifiState::Disconnected,
        (WifiState::Connected, "disconnect") => WifiState::Disconnected,
        (state, _) => state,
    }
}

#[test]
fn connected_disconnect_moves_to_disconnected() {
    assert_eq!(
        next_state(WifiState::Connected, "disconnect"),
        WifiState::Disconnected
    );
}
```
