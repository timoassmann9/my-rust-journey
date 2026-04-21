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

fn make_report(device_name: &str, readings: &[i32]) -> String {
    let mut high = false;
    let count = readings.len();
    for value in readings {
        if *value > 70 {
            high = true;
            break;
        }
    }
    format!("device={device_name}, count={count}, high={high}")
}

fn write_line(display: &mut String, line: &str) {
    display.push_str(format!("{line}\n").as_str());

    /*
    Alternativ:
    display.push_str(line);
    display.push('\n');
     */
}

fn clear_display(display: &mut String) {
    display.clear();
}

fn main() {
    let mut led_status = String::from("Status von LED1: ");
    append_status(&mut led_status, "ON");
    println!("{led_status}");

    let signals = vec![0, 20, 69, 30];
    let first_high = first_high_signal(&signals).unwrap_or(0);
    println!("First high: {first_high}");

    let readings = vec![1, 3, 69, 420, 44, 73];
    println!("Report: {}\n", make_report("esp32", &readings));

    let mut display_buf = String::from("hello!\n");
    println!("{display_buf}");
    write_line(&mut display_buf, "this is a new line");
    println!("{display_buf}");
    clear_display(&mut display_buf);
    println!("{display_buf}");
    println!("{}", display_buf.is_empty() == true);
}
