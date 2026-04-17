# Lesson 04: Error Handling, Modules, and Crates

## Goal

Learn how Rust represents failure explicitly and how projects are split into
modules and crates.

## Why this matters for your project

Your ESP32 code will interact with peripherals, parsing, network messages, and
possibly unreliable input. Error handling is part of the design, not an afterthought.

## Focus topics

- `Result<T, E>`
- `match` versus `?`
- defining small error enums
- splitting code into modules
- understanding what a crate is
