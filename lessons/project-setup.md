# External Cargo Project Setup

This repository does not commit lesson Cargo projects. You create them on your
own machine where Rust and Cargo work.

## Suggested location

Create the projects in a sibling folder next to this repository, for example:

```bash
mkdir -p ../rust-lesson-projects
cd ../rust-lesson-projects
```

## Commands to create the projects

```bash
cargo new lesson-01-rust-basics
cargo new lesson-02-ownership-borrowing
cargo new lesson-03-structs-enums-patterns
cargo new lesson-04-error-handling-modules-crates
cargo new lesson-05-testing-debugging-and-tooling-concepts
cargo new lesson-06-embedded-rust-foundations
cargo new lesson-07-esp32-peripherals-buttons-leds
cargo new lesson-08-i2c-and-display-abstractions
cargo new lesson-09-data-modeling-and-device-protocols
cargo new lesson-10-wifi-and-pc-communication
cargo new lesson-11-capstone-project
```

## How to use the mapping

For each lesson:

- read the lesson `README.md`
- create or edit code in that lesson's external Cargo project
- solve the exercises from the repo lesson folder
- only after finishing, compare with the paired solution and explanation files

## Project-to-file mapping

| External project | Lesson content in this repo |
| --- | --- |
| `lesson-01-rust-basics` | `lessons/01-rust-basics/README.md`, all files in `lessons/01-rust-basics/exercises/`, all files in `lessons/01-rust-basics/solutions/` |
| `lesson-02-ownership-borrowing` | `lessons/02-ownership-borrowing/README.md`, all files in `lessons/02-ownership-borrowing/exercises/`, all files in `lessons/02-ownership-borrowing/solutions/` |
| `lesson-03-structs-enums-patterns` | `lessons/03-structs-enums-patterns/README.md`, all files in `lessons/03-structs-enums-patterns/exercises/`, all files in `lessons/03-structs-enums-patterns/solutions/` |
| `lesson-04-error-handling-modules-crates` | `lessons/04-error-handling-modules-crates/README.md`, all files in `lessons/04-error-handling-modules-crates/exercises/`, all files in `lessons/04-error-handling-modules-crates/solutions/` |
| `lesson-05-testing-debugging-and-tooling-concepts` | `lessons/05-testing-debugging-and-tooling-concepts/README.md`, all files in `lessons/05-testing-debugging-and-tooling-concepts/exercises/`, all files in `lessons/05-testing-debugging-and-tooling-concepts/solutions/` |
| `lesson-06-embedded-rust-foundations` | `lessons/06-embedded-rust-foundations/README.md`, all files in `lessons/06-embedded-rust-foundations/exercises/`, all files in `lessons/06-embedded-rust-foundations/solutions/` |
| `lesson-07-esp32-peripherals-buttons-leds` | `lessons/07-esp32-peripherals-buttons-leds/README.md`, all files in `lessons/07-esp32-peripherals-buttons-leds/exercises/`, all files in `lessons/07-esp32-peripherals-buttons-leds/solutions/` |
| `lesson-08-i2c-and-display-abstractions` | `lessons/08-i2c-and-display-abstractions/README.md`, all files in `lessons/08-i2c-and-display-abstractions/exercises/`, all files in `lessons/08-i2c-and-display-abstractions/solutions/` |
| `lesson-09-data-modeling-and-device-protocols` | `lessons/09-data-modeling-and-device-protocols/README.md`, all files in `lessons/09-data-modeling-and-device-protocols/exercises/`, all files in `lessons/09-data-modeling-and-device-protocols/solutions/` |
| `lesson-10-wifi-and-pc-communication` | `lessons/10-wifi-and-pc-communication/README.md`, all files in `lessons/10-wifi-and-pc-communication/exercises/`, all files in `lessons/10-wifi-and-pc-communication/solutions/` |
| `lesson-11-capstone-project` | `lessons/11-capstone-project/README.md`, all files in `lessons/11-capstone-project/exercises/`, all files in `lessons/11-capstone-project/solutions/` |

## Recommended workflow per lesson

1. Create the lesson project with `cargo new`.
2. Read the lesson README and copy only the example code you need.
3. Implement the exercises in the external project.
4. Run `cargo check` often.
5. Add tests once the lesson starts covering testing or reusable logic.
6. Compare your code with the provided solution and explanation files.
