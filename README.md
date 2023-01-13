# Rust Attiny85 Template

This is a template to getting up and running with rust on the attiny85.

## Build command

`cargo build --target avr-specs/avr-attiny85.json --release`

Examine the machine code: 
`avr-objdump -d target/avr-attiny85/release/hal.elf`