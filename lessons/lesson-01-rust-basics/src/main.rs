use std::result;

fn parse_button_value(raw: &str) -> i32 {
    raw.trim().parse::<i32>().unwrap_or(-1)
}

fn led_label(is_on: bool) -> &'static str {
    if is_on { "LED is ON" }
    else { "LED is OFF" }
}

fn temperature_summary(readings: Vec<i32>) -> String {
    if readings.is_empty() { String::from("no readings") }
    else {
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
            sum += value;
        }

        let avg = sum / readings.len() as i32;
        format!("min={min}, max={max}, avg={avg}")
    }
}

fn blink_pattern(states: Vec<bool>) -> String {
    let mut result = String::new();
    for state in states {
        if state {
            result.push('*');
        }
        else {
            result.push('.');
        }
    }
    
    result
}

fn count_on(states: &[bool]) -> usize {
    let mut count = 0;
    for state in states {
        if *state { count += 1; }
    }

    count
}

fn main() {
    let button = "1";
    let button_int = parse_button_value(button);
    println!("button: {button_int}");

    let led_on = true;
    println!("{}", led_label(led_on));

    let readings1 = vec![3, 7, 22, 92, 4];
    let readings2 = vec![69, 420, 892, 4877, 9, 0];
    let empty: Vec<i32> = vec![];

    let readings1_result = temperature_summary(readings1);
    let readings2_result = temperature_summary(readings2);
    let empty = temperature_summary(empty);

    println!("First: {readings1_result}");
    println!("Second: {readings2_result}");
    println!("Third: {empty}");
}