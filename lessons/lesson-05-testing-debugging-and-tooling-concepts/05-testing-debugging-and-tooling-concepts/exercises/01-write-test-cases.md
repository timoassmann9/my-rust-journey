# Exercise 01: Write Test Cases

## Objective

Practice describing behavior precisely enough that it can be checked by code.

## Task

Write at least five tests for a function like:

```rust
fn next_state(current: WifiState, event: &str) -> WifiState
```

Include at least:

- one happy-path transition
- one failure-path transition
- one unknown-event case
- one case where the state should not change
- one case that proves a stable connected state

## Hints

- each test should prove one behavior
- give tests descriptive names
- if you are unsure whether a behavior matters, ask whether you would notice if it broke

## Why this exercise exists

Writing tests forces you to turn vague expectations into concrete rules. That is
valuable even before you run the tests.
