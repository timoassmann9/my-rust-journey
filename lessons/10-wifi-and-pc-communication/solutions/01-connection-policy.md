# Solution 01: Connection Policy

One reasonable beginner policy:

- retry up to 5 times
- display `wifi: retry 1/5`, `wifi: retry 2/5`, and so on
- after the last failure, enter an error state and stop pretending the device is usable

Possible code direction:

```rust
impl ConnectionState {
    fn should_retry(&self) -> bool {
        self.retry_count < 5
    }
}
```

Why this is a good start:

- it is explicit
- it is easy to explain and test
- it avoids infinite silent retry loops
