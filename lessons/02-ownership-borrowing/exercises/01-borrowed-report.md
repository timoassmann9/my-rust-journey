# Exercise 01: Borrowed Report

## Objective

Practice designing a function that reads borrowed data and returns newly created
owned output.

## Task

Write:

```rust
fn make_report(device_name: &str, readings: &[i32]) -> String
```

Expected output style:

`device=esp32-a, count=4, high=true`

## Constraints

- do not take ownership of `device_name`
- do not take ownership of the readings collection
- `high=true` means at least one reading is greater than `70`

## Hints

- use `readings.len()` for the count
- a simple loop with a boolean flag is fine here
- the report itself should be a fresh `String`

## Why this exercise exists

This pattern is everywhere in Rust: borrowed input, owned output. It is common
in formatting, logging, and protocol encoding.
