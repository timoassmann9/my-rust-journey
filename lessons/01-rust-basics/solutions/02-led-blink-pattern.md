# Solution 02: LED Blink Pattern

Build the output string step by step.

```rust
fn blink_pattern(states: Vec<bool>) -> String {
    let mut output = String::new();

    for state in states {
        if state {
            output.push('*');
        } else {
            output.push('.');
        }
    }

    output
}
```

Possible "count on" extension:

```rust
fn count_on(states: &[bool]) -> usize {
    let mut count = 0;

    for state in states {
        if *state {
            count += 1;
        }
    }

    count
}
```

Why this matters:

- it is the same kind of translation you will do later when turning hardware
  states into displayable or transmittable values
