# `wlug` graphical example
This example demonstrates `wlug`'s smooth integration and hot reloading capabilities.

You can press the spacebar to hot reload the plugins while the program is running.

# Build instructions
## Prerequisites
### Windows
- [Rust](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target for Rust: `rustup target add wasm32-unknown-unknown`

### Linux
- [Rust](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target for Rust: `rustup target add wasm32-unknown-unknown`
- Dependencies required by the `minifb` crate (used for rendering):
```console
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```

## Instructions
- The instructions below assume you are in the root of the repository

### Windows
```console
  $ ./build.ps1
  $ cargo run
```

### Linux
```console
  $ chmod +x ./build.sh
  $ ./build.sh
  $ cargo run
```
