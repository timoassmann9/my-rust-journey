# Exercise 01: Polling Loop Design

## Objective

Practice breaking embedded behavior into readable, testable steps.

## Task

Design or implement a function that represents one loop iteration:

- read a button state
- update an LED state
- write a message into a display buffer

You do not need real hardware APIs. Placeholder values or mock functions are fine.

## Constraints

- keep one loop iteration separate from the outer infinite loop idea
- avoid putting all logic in `main`
- make the flow readable enough that another person could explain it back to you

## Why this exercise exists

A lot of embedded code becomes messy because "the loop" is treated as one giant
thing. This exercise trains you to decompose it.
