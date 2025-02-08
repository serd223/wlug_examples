#!/bin/sh
cd ../plugs/test_plug
cargo build --release --target=wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/test_plug.wasm ../../embed
cd ../../embed
cd ../plugs/test_plug_c
clang --target=wasm32 --no-standard-libraries -Wl,--export-all -Wl,--no-entry -Wl,--allow-undefined -o test_plug_c.wasm test_plug_c.c
cp ./test_plug_c.wasm ../../embed
cd ../../embed
