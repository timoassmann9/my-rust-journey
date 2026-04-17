# Lesson 06: Embedded Rust Foundations

## Goal

Understand the mental shift from normal desktop Rust to embedded Rust.

## Why this matters for your project

An ESP32 is not a laptop. You will care about memory limits, timing, hardware
pins, and communication protocols. This lesson prepares you for that shift
without requiring actual board code yet.

## Focus topics

- `std` versus `no_std` as a concept
- resource constraints
- hardware abstraction
- polling versus interrupts
- state machines

## Practical rule

Before touching real hardware APIs, learn to model device behavior in plain Rust.
That reduces confusion dramatically.
