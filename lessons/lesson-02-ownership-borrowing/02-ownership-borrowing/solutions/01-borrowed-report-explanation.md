# Explanation 01: Borrowed Report

This function borrows everything it reads and only allocates for the final
report string.

That makes the ownership story very clean:

- the caller keeps the device name
- the caller keeps the readings
- the function creates only the output text

This is one of the first Rust patterns that starts to feel "native." If you can
look at a function signature and tell who owns what, you are making progress.
