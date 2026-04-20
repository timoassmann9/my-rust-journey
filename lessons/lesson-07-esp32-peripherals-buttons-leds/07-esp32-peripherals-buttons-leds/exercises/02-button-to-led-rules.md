# Exercise 02: Button to LED Rules

## Objective

Extend a controller without turning it into a mess of conditionals.

## Task

Create or extend a `Controller` type so that:

- `Released` does not toggle the LED
- each accepted `Pressed` event toggles the LED once
- the controller also counts valid presses

## Constraints

- keep the behavior in methods on the controller
- print the controller state after a short simulated event sequence

## Why this exercise exists

This exercise is a small example of event-driven logic with state, which is the
core of many device controllers.
