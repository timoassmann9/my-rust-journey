# Lesson 09: Data Modeling and Device Protocols

## Learning goal

Design clear message types before you care about sockets, Wi-Fi libraries, or
byte-level transport details.

## Why this matters for the ESP32 project

The device and the computer are two parts of one system. If their messages are
unclear, inconsistent, or loosely defined, debugging becomes much harder.

## Key concepts

- enums for message categories
- readable protocol encoding
- parsing raw input into typed commands
- validating messages near the boundary of the system
