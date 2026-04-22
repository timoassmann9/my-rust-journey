#[derive(Debug)]
pub enum ConfigError {
    MissingValue,
    InvalidNumber,
    OutOfRange,
}