TARGET = wasm32-unknown-unknown

.PHONY: all
all: target/$(TARGET)/release/fd_prestat_get.wasm

%.wasm: src/*.rs
	cargo build --release --target $(TARGET)
