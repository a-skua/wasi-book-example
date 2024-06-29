.PHONY: all
all: target/wasm32-unknown-unknown/release/hello.wasm

%.wasm: src/lib.rs Cargo.toml
	cargo build --release --target wasm32-unknown-unknown
