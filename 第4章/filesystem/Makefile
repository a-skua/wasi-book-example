.PHONY: all
all: target/wasm32-wasi/release/filesystem.wasm

%.wasm: src/* wit/* Cargo.toml
	cargo component build --release
