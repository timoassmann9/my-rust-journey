# Exercise 02: Mutable Display Buffer

## Objective

Practice changing owned data in place through a mutable reference.

## Task

Write:

```rust
fn write_line(display: &mut String, line: &str)
fn clear_display(display: &mut String)
```

Behavior:

- `write_line` appends the line and then a newline
- `clear_display` empties the existing buffer

## Constraints

- do not return a new string from either function
- keep ownership with the caller
- call both functions from `main` so you can observe the buffer before and after clearing it

## Hints

- `push_str` and `push` are enough
- `clear()` empties a `String`
- think of the buffer as a reusable resource, not a throwaway value

## Why this exercise exists

In embedded and systems programming, reusing buffers is common. This is a small
version of that pattern.
