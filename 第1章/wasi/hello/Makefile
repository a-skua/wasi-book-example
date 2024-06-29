TARGET=wasm32-wasip1

.PHONY: all
all: target/$(TARGET)/release/hello.wasm

%.wasm: src/main.rs Cargo.toml
	cargo build --release --target $(TARGET)
