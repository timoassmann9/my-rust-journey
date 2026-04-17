# Explanation 01: Add Screen Message

The best part of this solution is the function signature. It makes the rendering
requirements obvious:

- display target
- Wi-Fi status
- LED status
- button status

That means the function stays deterministic and easy to test with a fake display.
