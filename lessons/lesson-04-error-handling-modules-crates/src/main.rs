#[derive(Debug)]
enum ConfigError {
    MissingValue,
    InvalidNumber,
    OutOfRange,
}

fn parse_port(raw: &str) -> Result<u16, ConfigError> {
    if raw.is_empty() { return Err(ConfigError::MissingValue); }
    match raw.trim().parse::<u32>() {
        Ok(x) =>  {
            if u16::MIN as u32 <= x && x <= u16::MAX as u32 { Ok(x as u16) }
            else { Err(ConfigError::OutOfRange) }
        },
        Err(_) => { Err(ConfigError::InvalidNumber) }
    }
}

fn main() {
    let vals = vec!["2", "", "300", "-16 ", "70000", "abc"];
    for val in vals {
        let result = parse_port(val);
        println!("{result:?}");
    }
}
