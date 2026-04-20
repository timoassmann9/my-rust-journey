# Lesson 01: Rust Basics

## Learning goal

Build a foundation in variables, functions, control flow, strings, and vectors.
This lesson is deliberately simple because the next lessons depend on you being
comfortable with plain Rust values before ownership and borrowing are added on top.

## Why this matters for the ESP32 project

Your final project is hardware-focused, but most of the difficulty is still in
how you structure data and behavior. Before you model buttons, screens, and
Wi-Fi messages, you need to be able to:

- store state in variables
- branch on conditions
- loop through data
- build and return strings
- split logic into functions

## What to set up in your external Cargo project

Create `lesson-01-rust-basics` with `cargo new`, then work mainly in
`src/main.rs`.

## Key concepts

- `let` creates a variable
- `mut` allows reassignment
- function parameters and return types are explicit
- `if` is an expression, not only a statement
- `Vec<T>` stores a growable list of values
- `String` is owned text data you can build and modify

## Python to Rust comparison

- Python lets you change value types freely at runtime. Rust does not.
- Python often encourages "try it and see." Rust asks you to be explicit earlier.
- Python lists and strings feel effortless. Rust gives you more control, but you
  must understand what type you are working with.

## Worked example

```rust
fn parse_button_value(raw: &str) -> i32 {
    raw.trim().parse::<i32>().unwrap_or(-1)
}

fn led_label(is_on: bool) -> &'static str {
    if is_on {
        "LED is ON"
    } else {
        "LED is OFF"
    }
}
```

What to notice:

- `raw: &str` means the function reads borrowed text
- `parse::<i32>()` tries to turn text into a number
- `unwrap_or(-1)` provides a fallback value
- the `if` expression returns one of two string slices

## Common mistakes to watch for

- forgetting `mut` when you need to change a variable
- mixing up `String` and `&str`
- trying to return different types from different branches
- writing everything inside `main` instead of using helper functions

## Study sequence

1. Recreate the worked example in your external lesson project.
2. Add a few extra inputs and print the results.
3. Solve exercise 1 before exercise 2.
4. After solving, compare your answer with the solution file.
5. Then read the explanation file and rewrite your code if you learned a cleaner approach.
