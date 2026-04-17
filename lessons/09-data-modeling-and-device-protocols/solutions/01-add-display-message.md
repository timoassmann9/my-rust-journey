# Solution 01: Add Display Message

```rust
enum DeviceMessage {
    ButtonPressed,
    ButtonReleased,
    LedStatus { on: bool },
    DisplayText(String),
}

fn encode_message(message: &DeviceMessage) -> String {
    match message {
        DeviceMessage::ButtonPressed => "button:pressed".to_string(),
        DeviceMessage::ButtonReleased => "button:released".to_string(),
        DeviceMessage::LedStatus { on } => format!("led:{on}"),
        DeviceMessage::DisplayText(text) => format!("display:{text}"),
    }
}
```

Why a readable format is fine here:

- for a learning project, human-readable protocols are easier to debug than
  compact binary formats
