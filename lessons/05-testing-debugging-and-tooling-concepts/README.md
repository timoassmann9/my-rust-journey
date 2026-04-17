# Lesson 05: Testing, Debugging, and Tooling Concepts

## Goal

Learn how to think about verification in Rust, even though I cannot run `cargo`
commands on this machine.

## Why this matters for your project

Hardware bugs are painful to debug when basic logic is already wrong. Test as
much logic as possible away from the hardware.

## Focus topics

- what unit tests look like in Rust
- what to test before touching hardware
- `Debug` output and logging mindset
- narrowing a bug with smaller functions

## Constraint

I am not running `cargo test` here. Treat this lesson as design and practice for
tests you will run on your own machine.
