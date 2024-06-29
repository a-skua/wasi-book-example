.PHONY: all
all: target/wasm32-wasi/release/hello.wasm

%.wasm: src/* wit/* Cargo.toml
	cargo component build --release
