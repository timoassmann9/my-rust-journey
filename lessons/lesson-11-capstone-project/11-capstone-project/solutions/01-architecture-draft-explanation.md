# Explanation 01: Architecture Draft

This design separates three things that are often mixed together by beginners:

- long-lived application state
- incoming events
- host commands

That separation makes later control flow and testing much easier.
