# Lesson 07: ESP32 Peripherals, Buttons, and LEDs

## Learning goal

Model the smallest useful hardware interaction in your project: a button affects
an LED.

## Why this matters for the ESP32 project

This is the first place where device input and output meet. If you can structure
button handling clearly, the rest of the project becomes easier to design.

## Key concepts

- input versus output responsibilities
- edge cases like button bounce
- event handling
- keeping control logic in a dedicated type

## Practical rule

Do not let raw button reads control the whole program directly. Route them
through a small controller or state type first.
