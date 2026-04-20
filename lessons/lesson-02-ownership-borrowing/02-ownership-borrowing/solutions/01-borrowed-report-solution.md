# Solution 01: Borrowed Report

```rust
fn make_report(device_name: &str, readings: &[i32]) -> String {
    let mut high = false;

    for value in readings {
        if *value > 70 {
            high = true;
            break;
        }
    }

    format!(
        "device={device_name}, count={}, high={high}",
        readings.len()
    )
}
```
