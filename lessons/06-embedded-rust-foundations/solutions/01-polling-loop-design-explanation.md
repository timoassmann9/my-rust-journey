# Explanation 01: Polling Loop Design

The smart choice here is to model one iteration instead of the full endless
loop. That keeps the core behavior easy to reason about.

Once one iteration is clean, the outer loop becomes boring infrastructure. That
is exactly what you want.
