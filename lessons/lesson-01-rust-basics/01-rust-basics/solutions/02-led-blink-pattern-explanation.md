# Explanation 02: LED Blink Pattern

The important idea here is translation: the program stores one kind of value,
but humans or other systems often need a different representation.

`blink_pattern` converts `bool` values into characters:

- `true` becomes `*`
- `false` becomes `.`

This style is useful later when you turn device state into:

- display text
- status lines
- protocol messages

The stretch function uses `&[bool]` instead of `Vec<bool>`. That is a good sign
you are starting to separate "I need to read this data" from "I need to own this
data." That distinction becomes central in the next lesson.
