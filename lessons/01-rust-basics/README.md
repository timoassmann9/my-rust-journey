# Lesson 01: Rust Basics

## Goal

Build a usable foundation in variables, functions, control flow, strings, and
collections. This is the minimum Rust you need before ownership starts to make
sense.

## Why this matters for your project

Even hardware code is still code. Before you can reason about an LED state or a
message received over Wi-Fi, you need to be comfortable with values, branching,
loops, and basic standard library types.

## Python to Rust

- Python variables are flexible and dynamic at runtime. Rust variables have
  concrete static types.
- Python strings are easy to pass around. In Rust, strings come in several
  forms, and you will soon care about who owns them.
- Python often hides errors until runtime. Rust tries to make mistakes obvious
  earlier.

## Focus topics

- `let`, `mut`, and basic types
- `if`, `match`, `loop`, `while`, `for`
- functions and return values
- `String` and `Vec`
- simple parsing and formatting

## Study approach

Read the starter code in `src/main.rs`, then solve the exercises. Do not
optimize for "shortest code"; optimize for readable code that you understand.
