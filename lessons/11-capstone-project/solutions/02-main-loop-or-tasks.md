# Solution 02: Main Loop or Tasks

Polling-loop version:

1. Read button state.
2. Read any incoming Wi-Fi message.
3. Convert raw inputs into typed events.
4. Update `AppState`.
5. Apply outputs:
   - LED output
   - display output
   - outgoing Wi-Fi messages
6. If a critical error occurs, switch to an error state and display it clearly.

Why this is a good first version:

- it is easier to understand than jumping straight into concurrency
- each step can later become its own function

Task-based version later:

- input task
- network task
- state/update task
- output/render task
