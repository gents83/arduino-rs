
## Quickstart
You need a nightly Rust compiler for compiling Rust code for AVR. 
The correct version will be installed automatically due to the rust-toolchain.toml file.
Be sure to execute:
- rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc

## Install dependencies:
1) winget install AVRDudes.AVRDUDE ZakKemble.avr-gcc
2) cargo +stable install ravedude

## Starting your own project
The best way to start your own project is via the [`avr-hal-template`](https://github.com/Rahix/avr-hal-template) 
which you can easily use with [`cargo-generate`](https://github.com/cargo-generate/cargo-generate),
selecting the used Arduino board:

```bash
cargo install cargo-generate
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```

## Build Instructions
1. Run `cargo build` to build the firmware.

2. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

3. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

## References
- avr-hal: https://github.com/Rahix/avr-hal#readme
- ravedude: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.