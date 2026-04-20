# Exercise 02: LED Blink Pattern

## Objective

Translate internal program state into a compact, human-readable output format.

## Task

Write a function:

```rust
fn blink_pattern(states: Vec<bool>) -> String
```

Rules:

- `true` becomes `*`
- `false` becomes `.`

Example:

- input: `vec![true, false, true, true]`
- output: `"*.**"`

Then print the pattern for at least two different input sequences.

## Constraints

- build the string character by character
- avoid spaces in the output
- keep the function focused on conversion only

## Hints

- start with `String::new()`
- use `push` to add single characters
- if you want the stretch goal, write a second function instead of overloading the first one

## Stretch goal

Write `count_on(states: &[bool]) -> usize` and print the number of active LED states.

## Why this exercise exists

Embedded programs often turn internal state into short status displays, log
messages, or protocol messages. This is a small version of that skill.
