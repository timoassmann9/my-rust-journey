# Rust for an ESP32 Project

This lesson track is designed around one concrete outcome: building an ESP32
project in Rust that uses buttons, LEDs, an I2C screen, and Wi-Fi communication
with your computer.

The repository is now intentionally content-first. It does not contain committed
Cargo projects for the lessons. Instead, the lesson folders contain the teaching
material, exercises, solutions, and explanations, and separate setup files tell
you how to create the matching Cargo projects on your own machine.

## How to use this track

1. Work through the lessons in order.
2. Read each lesson `README.md` before writing code.
3. Create the external Cargo project for that lesson by following:
   - `lessons/project-setup.md`
   - `lessons/create-lesson-projects.sh`
4. Solve the exercises before opening the solution files.
5. Read the paired explanation files after reviewing the solutions.

## Important note about this machine

I cannot run `cargo` commands in this environment. Because of that, this repo
stores teaching content and project-creation instructions rather than verified
local Cargo projects.

On your own machine, you should create one Cargo project per lesson and use
`cargo check`, `cargo run`, and `cargo test` there while working through the
material.

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

- write and debug small to medium Rust programs without treating the compiler as an enemy
- reason about ownership, borrowing, enums, and error handling well enough to make design choices
- model hardware behavior using types, state machines, and explicit interfaces
- separate application logic from hardware and transport details
- design a simple protocol between an ESP32 and a computer
- sketch and then implement the core parts of your target ESP32 project

## Support files

- `project-setup.md`: the readable guide for creating the external Cargo projects
- `create-lesson-projects.sh`: the same setup commands in shell-script form
