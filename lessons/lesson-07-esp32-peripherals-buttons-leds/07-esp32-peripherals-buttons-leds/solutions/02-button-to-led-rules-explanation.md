# Explanation 02: Button to LED Rules

The controller type is the important design choice. It owns the behavior state:

- current LED status
- number of accepted presses

That is much clearer than scattering the same logic across `main`.
