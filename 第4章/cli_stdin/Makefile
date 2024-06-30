.PHONY: all
all: target/wasm32-wasi/release/cli_stdin.wasm

%.wasm: src/* wit/* Cargo.toml
	cargo component build --release
