#!/usr/bin/env bash
set -euo pipefail

# Create one external Cargo project per lesson.
# Run this script on your own machine, not inside this repo if you want to keep
# the lesson projects separate from the content repository.

cargo new lesson-01-rust-basics
# Files: lessons/01-rust-basics/README.md and all files under exercises/ and solutions/

cargo new lesson-02-ownership-borrowing
# Files: lessons/02-ownership-borrowing/README.md and all files under exercises/ and solutions/

cargo new lesson-03-structs-enums-patterns
# Files: lessons/03-structs-enums-patterns/README.md and all files under exercises/ and solutions/

cargo new lesson-04-error-handling-modules-crates
# Files: lessons/04-error-handling-modules-crates/README.md and all files under exercises/ and solutions/

cargo new lesson-05-testing-debugging-and-tooling-concepts
# Files: lessons/05-testing-debugging-and-tooling-concepts/README.md and all files under exercises/ and solutions/

cargo new lesson-06-embedded-rust-foundations
# Files: lessons/06-embedded-rust-foundations/README.md and all files under exercises/ and solutions/

cargo new lesson-07-esp32-peripherals-buttons-leds
# Files: lessons/07-esp32-peripherals-buttons-leds/README.md and all files under exercises/ and solutions/

cargo new lesson-08-i2c-and-display-abstractions
# Files: lessons/08-i2c-and-display-abstractions/README.md and all files under exercises/ and solutions/

cargo new lesson-09-data-modeling-and-device-protocols
# Files: lessons/09-data-modeling-and-device-protocols/README.md and all files under exercises/ and solutions/

cargo new lesson-10-wifi-and-pc-communication
# Files: lessons/10-wifi-and-pc-communication/README.md and all files under exercises/ and solutions/

cargo new lesson-11-capstone-project
# Files: lessons/11-capstone-project/README.md and all files under exercises/ and solutions/
