# Exercise 02: State Machine Sketch

## Objective

Practice turning vague lifecycle ideas into an explicit state model.

## Task

Design an enum for the device lifecycle with at least:

- `Booting`
- `WaitingForWifi`
- `Ready`
- `Error`

Then write a transition function or a short transition table.

## Hints

- ask what event moves the system from one state to another
- do not try to model every possible detail yet
- the important part is that the system has one main lifecycle state at a time

## Why this exercise exists

State machines make embedded code easier to reason about. They also stop you
from replacing design with a pile of unrelated flags.
