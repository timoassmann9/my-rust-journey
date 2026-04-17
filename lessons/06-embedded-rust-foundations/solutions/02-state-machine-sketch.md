# Solution 02: State Machine Sketch

```rust
enum DeviceLifecycle {
    Booting,
    WaitingForWifi,
    Ready,
    Error(&'static str),
}
```

Possible transitions:

- `Booting -> WaitingForWifi` after hardware initialization succeeds
- `Booting -> Error(...)` if initialization fails
- `WaitingForWifi -> Ready` when the network connects
- `WaitingForWifi -> Error(...)` after repeated failures
- `Ready -> Error(...)` if a critical subsystem stops working

Why this helps:

- the device can only be in one main lifecycle state at a time
- transitions become visible instead of being hidden in scattered conditions
