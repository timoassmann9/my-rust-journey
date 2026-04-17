# Lesson 08: I2C and Display Abstractions

## Learning goal

Represent screen behavior in a way that keeps your application logic separate
from hardware transport details.

## Why this matters for the ESP32 project

Your app should decide what to show. A lower layer should decide how that gets
sent over I2C. If those concerns are mixed together, changing the display code
becomes harder than necessary.

## Key concepts

- trait-based abstraction
- command-oriented rendering
- fake or test displays for logic development
- separating display content from device transport
