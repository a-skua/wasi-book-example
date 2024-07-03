TARGET = wasm32-unknown-unknown

.PHONY: all
all: target/$(TARGET)/release/cargo-crate.wasm

%.wasm: src/*.rs
	cargo build --release --target $(TARGET)
