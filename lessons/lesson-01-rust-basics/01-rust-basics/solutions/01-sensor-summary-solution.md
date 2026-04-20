# Solution 01: Sensor Summary

```rust
fn temperature_summary(readings: Vec<i32>) -> String {
    if readings.is_empty() {
        return "no readings".to_string();
    }

    let mut min = readings[0];
    let mut max = readings[0];
    let mut sum = 0;

    for value in &readings {
        if *value < min {
            min = *value;
        }
        if *value > max {
            max = *value;
        }
        sum += *value;
    }

    let avg = sum / readings.len() as i32;
    format!("min={min}, max={max}, avg={avg}")
}
```
