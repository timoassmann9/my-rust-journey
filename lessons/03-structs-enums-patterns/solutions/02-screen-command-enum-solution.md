# Solution 02: Screen Command Enum

```rust
enum ScreenCommand {
    Clear,
    WriteLine(String),
    SetBrightness(u8),
}

fn command_name(command: &ScreenCommand) -> &'static str {
    match command {
        ScreenCommand::Clear => "Clear",
        ScreenCommand::WriteLine(_) => "WriteLine",
        ScreenCommand::SetBrightness(_) => "SetBrightness",
    }
}
```
