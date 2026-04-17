# Solution 02: Host and Device Responsibilities

One clean split:

- ESP32:
  - read button input
  - control LED output
  - render simple display status
  - send events and accept simple commands
- Computer:
  - store logs
  - provide richer UI or analysis
  - decide higher-level behavior if needed

Typical message flow:

- device to host: button events, LED status, error messages
- host to device: LED commands, display commands, config updates

Why this split works:

- the embedded side stays focused and small
- the computer can handle more complex work more easily
