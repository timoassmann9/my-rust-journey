// Hello World
// fn main() {
//     println!("Hello, world!");
//     println!("I'm a Rustacean!");
// }

// Format
// fn main() {
//     let pi = 3.141592;
//     println!("Pi is roughly {pi:.5}");
// }

fn main() {
    let name = "Timo";
    let mut count = 0;

    for _ in 1..=5 {
        count += 1;
        println!("Hello, {}! (count: {})", name, count);
    }
    let parity = match count % 2 {
        0 => "even",
        _ => "odd",
    };
    println!("Final count {} is {}", count, parity);
}