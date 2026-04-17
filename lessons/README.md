# Rust for an ESP32 Project

This repository now contains a lesson track aimed at one concrete outcome:
building an ESP32 project in Rust that uses buttons, LEDs, an I2C screen, and
Wi-Fi communication with a computer.

## How to use this track

1. Work through the lessons in order.
2. Read each `README.md` first.
3. Solve the exercises in `exercises/` before opening `solutions/`.
4. Edit the Rust code in `src/main.rs` for each lesson.

## Important note about this machine

I cannot run `cargo` commands in this environment, so I have structured the
lessons and workspace without local verification on this machine.
When you work through them on a machine where `cargo` works, use commands like:

- `cargo run -p lesson-01-rust-basics`
- `cargo run -p lesson-07-esp32-peripherals-buttons-leds`
- `cargo check --workspace`
- `cargo test --workspace`

Each lesson is its own Cargo package inside the workspace.

## Lesson order

- `01-rust-basics`
- `02-ownership-borrowing`
- `03-structs-enums-patterns`
- `04-error-handling-modules-crates`
- `05-testing-debugging-and-tooling-concepts`
- `06-embedded-rust-foundations`
- `07-esp32-peripherals-buttons-leds`
- `08-i2c-and-display-abstractions`
- `09-data-modeling-and-device-protocols`
- `10-wifi-and-pc-communication`
- `11-capstone-project`

## What you should know by the end

You should be able to:

- read and write small to medium Rust programs without copying patterns blindly
- reason about ownership, borrowing, enums, and error handling
- model hardware behavior with Rust types and state machines
- structure an embedded-oriented Rust project
- design a simple protocol between an ESP32 and a computer
- sketch and implement the core pieces of your target ESP32 project
