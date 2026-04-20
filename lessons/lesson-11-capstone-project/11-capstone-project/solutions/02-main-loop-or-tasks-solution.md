# Solution 02: Main Loop or Tasks

Polling-loop draft:

1. Read button input.
2. Read any incoming Wi-Fi message.
3. Convert raw inputs into typed events.
4. Update `AppState`.
5. Apply outputs to LED, display, and outgoing messages.
6. If a critical error occurs, switch to an error state and report it clearly.
