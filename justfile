wasm_release_dir := "target/wasm32-unknown-unknown/wasm-release"

build_untitled_game_wasm:
    cargo build --package untitled_game --bin untitled_game --profile wasm-release --target wasm32-unknown-unknown
    wasm-opt -Os --output {{wasm_release_dir}}/untitled_game_optimized.wasm {{wasm_release_dir}}/untitled_game.wasm

build_untitled_game:
    cargo build --package untitled_game --bin untitled_game --release
