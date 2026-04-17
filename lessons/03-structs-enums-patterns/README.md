# Lesson 03: Structs, Enums, and Pattern Matching

## Learning goal

Move from primitive values to proper models. This is where Rust starts to feel
like a language for designing systems rather than only writing functions.

## Why this matters for the ESP32 project

Your project has real states and events:

- button pressed or released
- LED on or off
- display commands
- Wi-Fi connected, retrying, or failed

If you keep these as loose booleans and integers, the code becomes fragile fast.
Structs and enums make the program say what it means.

## Key concepts

- `struct` for grouping related state
- `enum` for one-of-many possibilities
- `impl` blocks for methods
- `match` for explicit branch handling

## Worked example

```rust
struct Led {
    pin: u8,
    is_on: bool,
}

enum ButtonEvent {
    Pressed,
    Released,
}
```

This already tells a clearer story than passing around unrelated values. The
rest of the lesson is about pushing that clarity further.

## Common mistakes to watch for

- storing unrelated booleans instead of designing a real type
- using strings for states that should be enums
- writing large `if` chains where `match` would be clearer

## Study sequence

1. Define the types first.
2. Only then add methods and formatting logic.
3. For each exercise, ask yourself whether the type design still makes sense if the project grows.
