.PHONY: all
all: target/wasm32-wasi/release/cli_run.wasm

%.wasm: src/* wit/* Cargo.toml
	cargo component build --release
