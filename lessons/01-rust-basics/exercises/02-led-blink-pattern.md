# Exercise 02: LED Blink Pattern

Add a function named `blink_pattern` that converts a list of booleans into a
compact string:

- `true` becomes `*`
- `false` becomes `.`

Example:

- input: `vec![true, false, true, true]`
- output: `"* . **"` is not allowed
- output should be `"*.**"`

Then print the pattern for at least two different sequences in `main`.

Stretch requirement:

- also print how many times the LED was on

What this trains:

- iterating over vectors
- string building
- turning state into a simple display format
