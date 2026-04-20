# Explanation 01: Device State

The main improvement here is not the `summary` method. It is the existence of
`DeviceState` itself.

Without the struct, every function call that needs these three pieces of data
would need three parameters. That gets harder to read and easier to misuse.

This is the beginning of a habit that matters a lot in Rust: design the data
shape first, then let the functions and methods follow from that design.
