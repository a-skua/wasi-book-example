.PHONY: all
all: target/wasm32-wasi/release/http_proxy.wasm

%.wasm: src/* wit/* Cargo.toml
	cargo component build --release
