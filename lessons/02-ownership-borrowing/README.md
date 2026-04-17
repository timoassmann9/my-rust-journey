# Lesson 02: Ownership and Borrowing

## Goal

Learn the core Rust idea that values have owners, and references let you use
data without taking ownership of it.

## Why this matters for your project

Embedded systems care about memory, state, and clear data flow. Ownership is not
just a Rust rule you have to satisfy. It helps you model who controls a buffer,
who can read from it, and who is allowed to mutate it.

## Focus topics

- moves
- borrowing with `&T`
- mutable borrowing with `&mut T`
- slices like `&str` and `&[T]`
- common compiler errors around moved values

## Python to Rust

Python references are implicit and easy to forget about. Rust makes access rules
explicit. That feels strict at first, but it prevents a class of state bugs that
are very annoying in larger programs.
