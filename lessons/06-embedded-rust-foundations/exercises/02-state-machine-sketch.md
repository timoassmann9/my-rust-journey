# Exercise 02: State Machine Sketch

Design an enum for the boot lifecycle of your ESP32 project.

Include at least:

- `Booting`
- `WaitingForWifi`
- `Ready`
- `Error`

Then write a function or pseudocode transition table that explains how the
device moves between these states.

What this trains:

- explicit state modeling
- avoiding "random booleans everywhere"
- making later code easier to test
