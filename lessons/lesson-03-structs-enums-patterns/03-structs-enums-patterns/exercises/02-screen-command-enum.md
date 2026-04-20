# Exercise 02: Screen Command Enum

## Objective

Use a data-carrying enum to represent screen actions explicitly.

## Task

Create:

- `Clear`
- `WriteLine(String)`
- `SetBrightness(u8)`

Then write:

```rust
fn command_name(command: &ScreenCommand) -> &'static str
```

Return only the variant name.

## Constraints

- match on a borrowed command
- do not allocate for `command_name`
- if you do the stretch goal, keep it as a second function

## Stretch goal

Write a human-readable formatter for the full command, including any stored data.

## Why this exercise exists

Embedded projects often act on commands. Enums let you model those commands
without magic strings spread all over the program.
