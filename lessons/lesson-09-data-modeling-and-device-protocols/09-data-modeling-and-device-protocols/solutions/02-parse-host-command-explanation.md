# Explanation 02: Parse Host Command

Returning `Option<HostCommand>` is a reasonable choice here because unknown
commands are expected bad input, not necessarily catastrophic failures.

The real win is that the rest of the program can work with typed commands rather
than raw strings.
