# Exercise 01: Borrowed Report

Write a function:

`fn make_report(device_name: &str, readings: &[i32]) -> String`

It should return a string like:

`device=esp32-a, count=4, high=true`

Rules:

- do not take ownership of the device name
- do not take ownership of the readings vector
- `high=true` means at least one reading is greater than `70`

What this trains:

- borrowed string slices
- borrowed slices
- returning owned output from borrowed input
