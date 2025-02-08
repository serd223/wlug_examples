function build_rust($name) {
    cd ..\plugs\$name 
    cargo build --release --target=wasm32-unknown-unknown
    cp .\target\wasm32-unknown-unknown\release\$name.wasm ..\..\embed
    cd ..\..\embed
}

function build_c($name) {
    cd ..\plugs\$name 
    clang --target=wasm32 --no-standard-libraries -Wl','--export-all -Wl','--no-entry -Wl','--allow-undefined -o "$name.wasm" "$name.c"
    cp "$name.wasm" ..\..\embed
    cd ..\..\embed
}

build_rust("test_plug")
build_c("test_plug_c")

