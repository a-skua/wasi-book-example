.PHONY: all
all: target/wasm32-wasi/release/sockets_tcp.wasm

%.wasm: src/* wit/* Cargo.toml
	cargo component build --release
