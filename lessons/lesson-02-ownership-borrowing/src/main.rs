fn append_status(message: &mut String, suffix: &str) {
    message.push_str(suffix);
}

fn first_high_signal(readings: &[i32]) -> Option<i32> {
    for value in readings {
        if *value > 50 {
            return Some(*value);
        }
    }
    None
}

fn main() {
    let mut led_status = String::from("Status von LED1: ");
    append_status(&mut led_status, "ON");
    println!("{led_status}");

    let signals = vec![0, 20, 69, 30];
    let first_high = first_high_signal(&signals).unwrap_or(0);
    println!("First high: {first_high}");
}
