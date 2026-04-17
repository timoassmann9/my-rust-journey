# Exercise 01: Device State

Create a struct named `DeviceState` with these fields:

- `button_pressed: bool`
- `led_on: bool`
- `wifi_connected: bool`

Then implement a method:

`fn summary(&self) -> String`

Example output:

`button=false, led=true, wifi=true`

What this trains:

- grouping related state
- implementing methods with `&self`
- turning state into a readable report
