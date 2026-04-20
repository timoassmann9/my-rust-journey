# Lesson 10: Wi-Fi and PC Communication

## Learning goal

Turn your message and state-design work into a simple device-host architecture.

## Why this matters for the ESP32 project

The ESP32 is not the whole system. Your computer will either:

- receive device events
- send commands
- or do both

That means you need clear responsibilities and a clear connection policy.

## Key concepts

- connection lifecycle
- retry rules
- device-versus-host responsibilities
- keeping transport code separate from protocol logic
