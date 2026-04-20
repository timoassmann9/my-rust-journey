# Lesson 02: Ownership and Borrowing

## Learning goal

Understand the central Rust rule that every value has an owner, and learn how
borrowing lets you read or modify data without taking ownership of it.

## Why this matters for the ESP32 project

When you work with buffers, device state, configuration strings, and messages,
you need to know:

- who owns the data
- who is only allowed to read it
- who is allowed to change it

Ownership and borrowing are the reason Rust can be strict without relying on a
garbage collector.

## What to set up in your external Cargo project

Create `lesson-02-ownership-borrowing` and use `src/main.rs` for the examples
and exercises.

## Key concepts

- moving ownership into a function
- borrowing with `&T`
- mutable borrowing with `&mut T`
- slices like `&str` and `&[i32]`
- why a moved value cannot be used afterward

## Worked example

```rust
fn append_status(message: &mut String, suffix: &str) {
    message.push_str(suffix);
}

fn first_high_signal(readings: &[i32]) -> Option<i32> {
    for value in readings {
        if *value > 50 {
            return Some(*value);
        }
    }
    None
}
```

Notice the difference:

- `message: &mut String` means the function changes an existing string
- `suffix: &str` means the text is only borrowed
- `readings: &[i32]` means the function reads from a slice instead of owning the whole vector

## Common mistakes to watch for

- passing `String` when `&str` would be enough
- consuming a vector in a loop and then trying to use it again
- trying to hold both mutable and immutable references at the same time
- treating borrowing as only a compiler rule instead of a design tool

## Study sequence

1. Recreate the worked example.
2. Change the input values and confirm you understand what is borrowed and what is owned.
3. Solve the borrowed read-only exercise first.
4. Then solve the mutable buffer exercise.
