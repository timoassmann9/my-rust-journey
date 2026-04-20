# Lesson 04: Error Handling, Modules, and Crates

## Learning goal

Learn how Rust models failure explicitly and how to split a project into
manageable parts before it turns into one large file.

## Why this matters for the ESP32 project

Your final project can fail in many ways:

- invalid configuration values
- missing input
- display or network errors
- unexpected messages from the computer

Rust encourages you to describe those failure cases directly instead of hiding
them until runtime.

## Key concepts

- `Result<T, E>`
- small custom error enums
- using `?` to pass errors upward
- modules as a tool for organizing code
- crates as packages of Rust code

## Worked example

Start with parsing something simple, such as brightness or a port number. Then
decide which failures are actually different in meaning. That is what your error
type should represent.

## Common mistakes to watch for

- using `unwrap` in places where failure is normal
- mixing unrelated responsibilities into one module
- creating one huge error type before you know what failures matter
