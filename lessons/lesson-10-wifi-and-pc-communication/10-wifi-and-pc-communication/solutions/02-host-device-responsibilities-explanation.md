# Explanation 02: Host and Device Responsibilities

The embedded side should stay focused on immediate device work. The host can do
the heavier thinking because it has more resources and is easier to debug.

That split keeps the ESP32 code smaller and more maintainable.
