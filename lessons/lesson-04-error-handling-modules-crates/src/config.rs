use crate::error::ConfigError;

pub fn parse_port(raw: &str) -> Result<u16, ConfigError> {
    if raw.trim().is_empty() { return Err(ConfigError::MissingValue); }
    match raw.trim().parse::<u32>() {
        Ok(x) =>  {
            match u16::try_from(x) {
                Ok(x) => Ok(x),
                Err(_) => Err(ConfigError::OutOfRange),
            }
        },
        Err(_) => Err(ConfigError::InvalidNumber)
    }

    /*
    Alternative:

    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::MissingValue);
    }

    let parsed = trimmed
        .parse::<u32>()
        .map_err(|_| ConfigError::InvalidNumber)?;

    if parsed > u16::MAX as u32 {
        return Err(ConfigError::OutOfRange);
    }

    Ok(parsed as u16)
     */
}