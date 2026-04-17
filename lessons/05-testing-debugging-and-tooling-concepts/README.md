# Lesson 05: Testing, Debugging, and Tooling Concepts

## Learning goal

Learn how to reason about correctness and debugging in Rust before your code is
mixed with real hardware behavior.

## Why this matters for the ESP32 project

If you only discover logic bugs after flashing code to the board, debugging gets
slow and confusing. The smart move is to test and debug as much logic as you can
away from the hardware.

## Key concepts

- writing focused unit tests
- thinking in inputs and outputs
- using `Debug` output to inspect state
- isolating logic from hardware dependencies

## Constraint

I cannot run `cargo test` here, so this lesson is about designing and writing
tests that you run on your own machine.

## Worked example

A Wi-Fi state transition function is a good test target because:

- it is pure logic
- it has clear inputs and outputs
- it can be wrong in subtle ways

That makes it ideal for learning tests.
