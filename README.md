## mbus-read

A simple utility for generating Modbus RTU read_holding_registers commands.

### Usage

`mbus_read [MODBUS_ADDRESS] [STARTING_REGISTER] [NUM_OF_REGISTERS]`

E.g. `mbus_read 481 0 50`

The above command translates to: read 50 registers, starting at register 0, from unit with Modbus address 481.

`mbus_read -h` for help.

### Dependencies

[clap](https://crates.io/crates/clap) - Easily create commandline utilities and parse arguments

[futures](https://crates.io/crates/futures) - An implementation of futures and streams featuring zero allocations, composability, and iterator-like interfaces

[tokio-core](https://crates.io/crates/tokio-core) - Core I/O and event loop primitives for asynchronous I/O in Rust

[tokio-modbus](https://crates.io/crates/tokio-modbus) - A [tokio](https://tokio.rs/)-based Modbus library

[tokio-serial](https://crates.io/crates/tokio-serial) - A serial port implementation for tokio

### ARM Compilation

[Compiling on Raspberry Pi](https://stackoverflow.com/questions/29917513/how-can-i-compile-rust-code-to-run-on-a-raspberry-pi-2)

### To Do
