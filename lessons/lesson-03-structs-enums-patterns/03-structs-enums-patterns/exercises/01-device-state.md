# Exercise 01: Device State

## Objective

Practice grouping related device state and exposing a simple read-only method.

## Task

Create:

```rust
struct DeviceState {
    button_pressed: bool,
    led_on: bool,
    wifi_connected: bool,
}
```

Then implement:

```rust
fn summary(&self) -> String
```

Example output:

`button=false, led=true, wifi=true`

## Constraints

- keep the fields simple for now
- `summary` should not consume the struct
- print at least two different example states from `main`

## Hints

- `format!` is the easiest way to build the output
- this is a good place to use an `impl` block

## Why this exercise exists

Later lessons will add more realistic state, but this is the first step toward a
single type that represents the device instead of scattered values.
