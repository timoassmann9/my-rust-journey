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
    let mut status = String::from("boot");
    append_status(&mut status, " -> wifi ready");
    println!("{status}");

    let readings = vec![10, 20, 65, 30];
    println!("{:?}", first_high_signal(&readings));

    // Add experiments for ownership, borrowing, and mutation here.
}
