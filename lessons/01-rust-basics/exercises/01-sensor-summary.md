# Exercise 01: Sensor Summary

Open `src/main.rs`.

Write a function named `temperature_summary` that:

- takes a `Vec<i32>` of temperature readings
- returns a `String`
- includes the minimum, maximum, and average reading

Example output style:

`min=18, max=24, avg=21`

Constraints:

- do not panic on an empty vector
- for an empty vector, return `no readings`
- compute the average as integer division for now

What this trains:

- loops
- mutable variables
- return values
- building a `String`
