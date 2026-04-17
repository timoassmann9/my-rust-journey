# Exercise 02: Parse Host Command

Design an enum for commands sent from the computer to the ESP32.

Include at least:

- turn LED on
- turn LED off
- clear display

Then write:

`fn parse_host_command(raw: &str) -> Option<HostCommand>`

What this trains:

- turning text into structured control messages
- rejecting unknown input safely
