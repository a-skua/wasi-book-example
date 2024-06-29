.PHONY: all
all: hello.wasm

%.wasm: %.dart
	dart compile wasm -o $@ $<
