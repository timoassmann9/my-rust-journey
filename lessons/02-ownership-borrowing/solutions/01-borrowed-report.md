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

Why this shape is good:

- borrowed inputs mean the caller keeps ownership
- the function still returns an owned `String` because the report is new data
- using `readings.len()` avoids extra bookkeeping
