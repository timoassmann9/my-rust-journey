# Explanation 02: Mutable Display Buffer

The key idea is that mutable borrowing gives temporary permission to change a
value without transferring ownership.

That is different from returning a new string each time:

- the caller stays responsible for the buffer
- the functions are focused on behavior, not allocation strategy

Later, when you manage device buffers or network payloads, this pattern becomes
much more important.
