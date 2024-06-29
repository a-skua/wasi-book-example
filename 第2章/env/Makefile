TARGET = wasm32-wasip1

.PHONY: all
all: target/$(TARGET)/release/env.wasm

%.wasm: src/*.rs
	cargo build --release --target $(TARGET)
