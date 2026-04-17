# Exercise 01: Sensor Summary

## Objective

Practice looping over data, tracking multiple values at once, and building a
formatted string.

## Task

In your external `lesson-01-rust-basics` project, write a function:

```rust
fn temperature_summary(readings: Vec<i32>) -> String
```

The function should:

- return `no readings` for an empty vector
- otherwise report the minimum, maximum, and average value

Example output:

`min=18, max=24, avg=21`

## Constraints

- do not panic on empty input
- use integer division for the average
- write the logic yourself instead of using iterator helpers you do not understand yet

## Hints

- handle the empty case first
- once the vector is known to be non-empty, you can use the first value to initialize `min` and `max`
- you only need one loop to compute `min`, `max`, and `sum`

## Deliverable

- the function
- a few calls in `main` that print example outputs

## Why this exercise exists

You will later summarize sensor values, message state, and system status. This
exercise is the first step toward that kind of code.
