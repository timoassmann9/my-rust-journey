# Exercise 02: Parse Host Command

## Objective

Turn raw host input into a typed command enum that the device can act on safely.

## Task

Design a `HostCommand` enum that includes at least:

- turn LED on
- turn LED off
- clear display

Then write:

```rust
fn parse_host_command(raw: &str) -> Option<HostCommand>
```

## Hints

- trim the input first
- return `None` for unknown commands
- keep the text protocol readable for now

## Why this exercise exists

You want unsafe or malformed input to stop at the system boundary, not leak into
the rest of the program.
