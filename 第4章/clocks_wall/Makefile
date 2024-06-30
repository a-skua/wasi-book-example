.PHONY: all
all: target/wasm32-wasi/release/clocks_wall.wasm

%.wasm: src/* wit/* Cargo.toml
	cargo component build --release
