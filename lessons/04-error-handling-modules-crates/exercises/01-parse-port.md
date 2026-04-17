# Exercise 01: Parse Port

Write a function:

`fn parse_port(raw: &str) -> Result<u16, ConfigError>`

Use this error enum:

- `MissingValue`
- `InvalidNumber`
- `OutOfRange`

Rules:

- empty input becomes `MissingValue`
- non-numeric input becomes `InvalidNumber`
- numeric input above `65535` becomes `OutOfRange`

What this trains:

- explicit failure cases
- mapping parsing failures into your own error type
