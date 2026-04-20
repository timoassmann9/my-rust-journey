# Exercise 02: Split Into Modules

## Objective

Practice organizing code before the project becomes difficult to navigate.

## Task

Refactor your lesson project so that:

- parsing functions live in a `config` module
- the error enum lives in an `error` module
- `main` imports what it needs

You do not need a perfect production layout. The goal is to make the code easier
to read and easier to grow.

## Hints

- start small; two modules are enough
- move types before moving helper functions
- after the split, ask whether each file has one clear responsibility

## Why this exercise exists

Many beginner Rust projects become hard to read because every new idea lands in
one file. This exercise tries to break that habit early.
