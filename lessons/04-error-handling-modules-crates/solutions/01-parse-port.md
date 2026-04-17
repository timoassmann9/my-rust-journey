# Solution 01: Parse Port

```rust
#[derive(Debug)]
enum ConfigError {
    MissingValue,
    InvalidNumber,
    OutOfRange,
}

fn parse_port(raw: &str) -> Result<u16, ConfigError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::MissingValue);
    }

    let parsed = trimmed
        .parse::<u32>()
        .map_err(|_| ConfigError::InvalidNumber)?;

    if parsed > u16::MAX as u32 {
        return Err(ConfigError::OutOfRange);
    }

    Ok(parsed as u16)
}
```

Why parse to `u32` first:

- it lets you distinguish "valid number but too large" from "not a number"
