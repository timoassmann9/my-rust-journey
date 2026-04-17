# Solution 02: LED Blink Pattern

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
