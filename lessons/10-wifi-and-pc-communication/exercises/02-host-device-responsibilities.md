# Exercise 02: Host and Device Responsibilities

## Objective

Practice deciding where logic should live in a two-part system.

## Task

Write a short design note that explains:

- what logic belongs on the ESP32
- what logic belongs on the computer
- which messages flow from device to host
- which messages flow from host to device

Constraint:

- keep the ESP32 side simpler than the host side

## Why this exercise exists

Many first device projects overload the microcontroller with work that belongs on
the computer. This exercise is meant to prevent that.
