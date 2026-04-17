# Explanation 01: Parse Port

Parsing to `u32` first is the key move here. It lets the function distinguish:

- input that is not numeric
- input that is numeric but too large for a port

That is exactly the kind of distinction a custom error type is for. Good error
handling is not about making the code look advanced. It is about preserving
useful information.
