fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn classify(temp_f: f64) -> &'static str {
    if temp_f > 75.0 { "hot" } 
    else if temp_f > 60.0 { "mild" } 
    else { "cold" }
}

fn main() {
    // exercise 1
    // let name = "Timo";
    // let mut count: u32 = 0;

    // for _ in 1..=5 {
    //     count += 1;
    //     println!("Hello, {}! (count: {})", name, count);
    // }

    // let result = match count % 2 {
    //     0 => "even",
    //     _ => "odd",
    // };
    // println!("\nCount is {} after {} counts", result, count);

    // exercise 2
    let values = vec![0.0, 20.0, 35.0];
    for value in values {
        let fahrenheit = celsius_to_fahrenheit(value);
        println!("{value:<2}° Celsius ({fahrenheit:.2}° Fahrenheit) feels {}", classify(fahrenheit));
    }

    // exercise 3
    for i in 1..=30 {
        let result = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => i.to_string(),
        };
        println!("{result}");
    }
}
