# Explanation 01: Sensor Summary

This solution starts by handling the risky case first: an empty vector. That
prevents indexing into `readings[0]` when no values exist.

After that, the solution uses the first element as the initial `min` and `max`.
That is a common Rust pattern when you know the collection is non-empty.

The loop does three jobs at once:

- update the minimum
- update the maximum
- add to the running sum

This is not just efficient. It also teaches you to reason about state that
changes over time inside a loop.

Important detail:

- the loop iterates over `&readings`, which means the vector is borrowed rather
  than consumed
- that allows the code to call `readings.len()` after the loop

If you wrote a version that moved the vector in the loop and then could not use
it afterward, that is a useful mistake. It means you are already brushing up
against ownership, which is exactly what lesson 2 will address.
