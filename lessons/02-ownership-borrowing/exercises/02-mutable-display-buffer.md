# Exercise 02: Mutable Display Buffer

Write a function:

`fn write_line(display: &mut String, line: &str)`

Requirements:

- append `line` to `display`
- append a newline after the line
- do not create and return a new `String`

Then write another function:

`fn clear_display(display: &mut String)`

It should empty the existing buffer.

What this trains:

- mutable borrowing
- changing an existing owned value in place
- the difference between ownership and access
