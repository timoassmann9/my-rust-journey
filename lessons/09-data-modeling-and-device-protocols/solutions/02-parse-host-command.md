# Solution 02: Parse Host Command

```rust
enum HostCommand {
    LedOn,
    LedOff,
    ClearDisplay,
}

fn parse_host_command(raw: &str) -> Option<HostCommand> {
    match raw.trim() {
        "led:on" => Some(HostCommand::LedOn),
        "led:off" => Some(HostCommand::LedOff),
        "display:clear" => Some(HostCommand::ClearDisplay),
        _ => None,
    }
}
```

Why return `Option` here:

- unknown commands are expected input failures, not necessarily exceptional
  program crashes
