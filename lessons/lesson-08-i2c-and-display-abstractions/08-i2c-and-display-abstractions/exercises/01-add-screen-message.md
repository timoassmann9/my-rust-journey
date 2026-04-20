# Exercise 01: Add Screen Message

## Objective

Pass all state needed for rendering through the function interface.

## Task

Extend `show_status` so it also writes a third line for button state:

- `button: pressed`
- `button: released`

## Constraints

- use a function parameter, not a global
- clear the display before writing the lines
- keep the function responsible for formatting, not hardware details

## Why this exercise exists

This teaches a habit that matters everywhere in Rust: make the data dependency
explicit in the function signature.
