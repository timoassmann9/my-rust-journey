mod config;
mod error;

fn main() {
    let vals = vec!["2", "", "300", "-16 ", "70000", "abc"];
    for val in vals {
        let result = config::parse_port(val);
        println!("{result:?}");
    }
}
