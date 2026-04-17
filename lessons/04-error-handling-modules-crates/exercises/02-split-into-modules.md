# Exercise 02: Split Into Modules

Imagine this lesson is growing into a real project.

Refactor the code mentally or in files so that:

- parsing functions live in a `config` module
- the error enum lives in an `error` module
- `main` or a top-level entrypoint imports what it needs

You do not need a perfect Cargo layout here. The point is to practice how Rust
code is organized.

What this trains:

- separating responsibilities
- using modules to reduce chaos
- thinking ahead before the project gets large
