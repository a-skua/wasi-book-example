TARGET = wasm32-unknown-unknown

.PHONY: all
all: target/$(TARGET)/release/clock_time_get.wasm

%.wasm: src/lib.rs Cargo.toml
	cargo build --release --target $(TARGET)
