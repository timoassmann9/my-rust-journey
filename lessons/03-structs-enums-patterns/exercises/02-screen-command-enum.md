# Exercise 02: Screen Command Enum

Create an enum:

- `Clear`
- `WriteLine(String)`
- `SetBrightness(u8)`

Then write a function:

`fn command_name(command: &ScreenCommand) -> &'static str`

Return only the variant name, not its data.

Example:

- `WriteLine("Hi".to_string())` becomes `"WriteLine"`

Stretch goal:

- write a second function that turns the whole command into a human-readable
  string

What this trains:

- data-carrying enums
- matching on borrowed enum values
- separating control flow from stored data
