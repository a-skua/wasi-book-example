.PHONY: all
all: example.wasm

%.wasm: %.wat
	wasm-tools parse -o $@ $<
