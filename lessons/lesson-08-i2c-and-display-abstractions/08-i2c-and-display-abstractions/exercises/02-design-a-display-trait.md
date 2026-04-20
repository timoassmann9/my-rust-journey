# Exercise 02: Design a Display Trait

## Objective

Practice deciding what belongs in an abstraction boundary.

## Task

Add one new method to the display trait, such as:

- `set_brightness`
- `write_centered`
- `write_at(line, text)`

Then explain:

- why the method belongs in the trait
- what the application should know
- what the application should not know about I2C internals

## Why this exercise exists

Good abstraction is not about hiding everything. It is about exposing the right
things at the right level.
