# OS Dev
But in rust.
**Why Rust?** Because rust provides memory safety.

# Dependencies
- rustup
- cargo
- qemu

# Building
Install toolchain (if not exists):
```sh
rustup target add i686-unknown-linux-gnu
```
Run (automaticly spawns qemu x86 emulator):
```sh
cargo run
```
