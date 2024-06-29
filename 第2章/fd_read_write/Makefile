TARGET = wasm32-unknown-unknown

.PHONY: all
all: target/$(TARGET)/release/fd_read_write.wasm

%.wasm: src/lib.rs Cargo.toml
	cargo build --release --target $(TARGET)
