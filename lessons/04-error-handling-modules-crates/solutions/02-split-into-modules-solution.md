# Solution 02: Split Into Modules

One clean split is:

- `error.rs` for `ConfigError`
- `config.rs` for parsing functions
- `main.rs` for calling the parsing logic and printing results
