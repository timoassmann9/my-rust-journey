# Explanation 02: Screen Command Enum

This enum is better than a string-based command system for two reasons:

- only valid command shapes can exist
- the compiler helps you handle every variant

The `match` is also doing useful design work. It forces you to say what happens
for every case. That becomes even more important once commands carry real device
data.
