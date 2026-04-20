# Solution 01: Connection Policy

One simple policy:

- retry up to 5 times
- show `wifi: retry 1/5`, `wifi: retry 2/5`, and so on
- after the last failure, enter an error state

Possible helper:

```rust
impl ConnectionState {
    fn should_retry(&self) -> bool {
        self.retry_count < 5
    }
}
```
