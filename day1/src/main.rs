fn main() {
    // exercise 1
    let name = "Timo";
    let mut count: u32 = 0;

    for _ in 1..=5 {
        count += 1;
        println!("Hello, {}! (count: {})", name, count);
    }

    let result = match count % 2 {
        0 => "even",
        _ => "odd",
    };
    println!("\nCount is {} after {} counts", result, count);

    // exercise 2
    
}
