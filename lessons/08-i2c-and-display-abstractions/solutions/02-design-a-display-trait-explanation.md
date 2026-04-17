# Explanation 02: Design a Display Trait

`set_brightness` belongs in the trait because it describes a display capability,
not a transport detail.

Application code should know:

- that brightness can be requested

Application code should not know:

- which bytes are sent over I2C
- which address is used
- what command sequence the hardware driver needs internally
