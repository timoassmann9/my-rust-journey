# Exercise 01: Write Test Cases

Read `src/main.rs` and write down at least 5 test cases for `next_state`.

At minimum, include:

- one happy path
- one failure path
- one "unknown event" case
- one case where the state should not change
- one case that proves the final connected state stays stable

You can write the tests as Rust `#[test]` functions or as plain text first.

What this trains:

- test case design
- thinking in inputs and outputs
- treating state transitions as logic you can verify
