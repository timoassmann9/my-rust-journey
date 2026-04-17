#[derive(Debug)]
enum ConfigError {
    MissingValue,
    InvalidNumber,
}

fn parse_brightness(raw: &str) -> Result<u8, ConfigError> {
    if raw.trim().is_empty() {
        return Err(ConfigError::MissingValue);
    }

    raw.trim()
        .parse::<u8>()
        .map_err(|_| ConfigError::InvalidNumber)
}

fn main() {
    println!("{:?}", parse_brightness("42"));
    println!("{:?}", parse_brightness(""));
}
