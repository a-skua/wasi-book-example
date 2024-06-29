TARGET = wasm32-unknown-unknown

.PHONY: all
all: target/$(TARGET)/release/path_get.wasm

%.wasm: src/*.rs
	cargo build --release --target $(TARGET)
