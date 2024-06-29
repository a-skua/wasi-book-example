TARGET = wasm32-wasip1

.PHONY: all
all: target/$(TARGET)/release/time.wasm

%.wasm: src/*.rs
	cargo build --release --target $(TARGET)
