# Solution 02: Host and Device Responsibilities

One reasonable split:

- ESP32: read inputs, control outputs, report state, accept simple commands
- Computer: logging, richer UI, history, analysis, higher-level orchestration

Typical message flow:

- device to host: button events, LED status, Wi-Fi status, errors
- host to device: LED commands, display commands, configuration updates
