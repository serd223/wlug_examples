function build_rust($name) {
    cd plugs\$name 
    cargo build --release --target=wasm32-unknown-unknown
    cp .\target\wasm32-unknown-unknown\release\$name.wasm ..\..\
    cd ..\..\
}

build_rust("draw")
build_rust("draw_two")

