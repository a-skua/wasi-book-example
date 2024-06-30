.PHONY: all
all: target/wasm32-wasi/release/cli_env.wasm

%.wasm: src/* wit/* Cargo.toml
	cargo component build --release
