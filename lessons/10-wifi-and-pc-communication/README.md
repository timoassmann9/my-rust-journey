# Lesson 10: Wi-Fi and PC Communication

## Goal

Connect the protocol thinking from the previous lesson to a realistic device and
host architecture.

## Why this matters for your project

Your ESP32 is only half the system. The other half is the computer receiving
data or sending commands. Good boundaries matter more than any single library.

## Focus topics

- connection lifecycle
- sending versus receiving responsibilities
- retry behavior
- keeping protocol logic separate from transport code
