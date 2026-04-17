fn parse_button_value(raw: &str) -> i32 {
    raw.trim().parse::<i32>().unwrap_or(-1)
}

fn led_label(is_on: bool) -> &'static str {
    if is_on {
        "LED is ON"
    } else {
        "LED is OFF"
    }
}

fn main() {
    let samples = vec!["1", "0", "not-a-number", "3"];

    for raw in samples {
        let value = parse_button_value(raw);
        println!("raw={raw:?}, parsed={value}");
    }

    let states = vec![true, false, true];
    for state in states {
        println!("{}", led_label(state));
    }

    // Add your own experiments here while solving the exercises.
}
