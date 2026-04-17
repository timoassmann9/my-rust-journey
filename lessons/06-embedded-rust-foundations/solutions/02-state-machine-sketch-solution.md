# Solution 02: State Machine Sketch

```rust
enum DeviceLifecycle {
    Booting,
    WaitingForWifi,
    Ready,
    Error(&'static str),
}
```
