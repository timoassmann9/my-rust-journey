# Exercise 01: Parse Port

## Objective

Practice turning raw text into a validated value while preserving the reason for
failure.

## Task

Write:

```rust
fn parse_port(raw: &str) -> Result<u16, ConfigError>
```

Use an error enum with:

- `MissingValue`
- `InvalidNumber`
- `OutOfRange`

## Rules

- empty input becomes `MissingValue`
- non-numeric input becomes `InvalidNumber`
- numbers above `65535` become `OutOfRange`

## Hints

- parse to a larger numeric type first
- distinguish "not a number" from "valid number but too large"
- keep the error names meaningful instead of generic

## Why this exercise exists

You are practicing an important embedded and networking skill: rejecting bad
input without losing the reason it was rejected.
