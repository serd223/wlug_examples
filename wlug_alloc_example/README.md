# Proof of concept allocation example for `wlug`
The `embed` crate uses `wlug` to execute the plugins inside `plugs` and exposes a key-value style persistent storage API to them.

# Build instructions
## Prerequisites
### Windows
- [Rust](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target for Rust: `rustup target add wasm32-unknown-unknown`
- [clang](https://releases.llvm.org/download.html)

### Linux
- [Rust](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target for Rust: `rustup target add wasm32-unknown-unknown`
- [clang](https://releases.llvm.org/download.html) (compiling to wasm might require extra llvm tools depending on your distro and configuration)

## Instructions
- The instructions below assume you are in the root of the repository
- The instructions below build and run `embed`

### Windows
```console
  $ cd embed
  $ ./build.ps1
  $ cargo run
```

### Linux
```console
  $ cd embed
  $ chmod +x ./build.sh
  $ ./build.sh
  $ cargo run
```
