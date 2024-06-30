.PHONY: all
all: target/wasm32-wasi/release/cli_command.wasm

%.wasm: src/* Cargo.toml
	cargo component build --release
