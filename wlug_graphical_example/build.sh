#!/bin/sh
cd plugs/draw 
cargo build --release --target=wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/draw.wasm ../../
cd ../../
cd plugs/draw_two 
cargo build --release --target=wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/draw_two.wasm ../../
cd ../../
