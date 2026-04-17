# Exercise 02: Button to LED Rules

Extend `Controller` with one more behavior rule:

- a `Released` event should not toggle the LED
- two accepted `Pressed` events should toggle the LED twice

Then add another requirement:

- track how many valid presses have been seen

What this trains:

- stateful logic
- event handling
- adding behavior without losing clarity
