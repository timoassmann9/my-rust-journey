# Lesson 11: Capstone Project

## Goal

Combine the earlier lessons into a project design close to your actual target.

## Project brief

Design a Rust-based ESP32 application that:

- reads a button
- controls an LED
- updates an I2C display
- exchanges messages with a computer over Wi-Fi

## Suggested architecture

- one module for hardware-facing logic
- one module for device state
- one module for protocol messages
- one module for connection management

## Success criteria

By the end of this lesson, you should be able to explain:

- the core data types
- the state transitions
- the message protocol
- the main loop or task structure
- the likely failure cases
