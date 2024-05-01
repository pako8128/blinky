# Blinky

blinks an led on a nucleo f070rb development board.

## Prerequisites

* rust
* probe-rs

to install probe-rs try your package manager or use
```bash
cargo install probe-rs --locked --features cli
```

you will also need the thumbv6m-none-eabi toolchain
```bash
rustup target add thumbv6m-none-eabi
```

## Flash and Run

after that `cargo run` should do it
