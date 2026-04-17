# Lesson 06: Embedded Rust Foundations

## Learning goal

Understand the mental shift from desktop Rust to embedded Rust without getting
lost in board-specific APIs too early.

## Why this matters for the ESP32 project

An ESP32 is constrained. You care about:

- memory limits
- hardware pins
- timing
- device state
- communication with the outside world

If you jump directly into board code without understanding these constraints,
the project becomes harder than it needs to be.

## Key concepts

- `std` versus `no_std` as an idea
- resource awareness
- polling versus interrupts
- hardware abstraction
- explicit state machines

## Practical rule

Model the behavior in plain Rust first. That gives you something you can think
about and test before you add board-specific details.
