# Solution 02: Split Into Modules

One clean split is:

- `error.rs` contains `ConfigError`
- `config.rs` contains `parse_brightness` and `parse_port`
- `main.rs` declares `mod error;` and `mod config;`, then calls those functions

Why this split helps:

- parsing logic stays together
- error definitions stay reusable
- the entrypoint reads more like an overview than a dump of every detail
