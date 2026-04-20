# Lesson 11: Capstone Project

## Learning goal

Pull the earlier lessons into one project design that is close to the ESP32
system you actually want to build.

## Project brief

Design a Rust-based ESP32 application that:

- reads a button
- controls an LED
- updates an I2C display
- exchanges messages with a computer over Wi-Fi

## What a good capstone looks like

A good capstone is not just "code that exists." It should show:

- clear data types
- clear ownership of responsibilities
- readable control flow
- explicit state transitions
- basic failure handling

## Suggested architecture

- hardware-facing module
- device-state module
- protocol module
- connection-management module
- top-level app module that coordinates them
