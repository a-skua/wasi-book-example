.PHONY: all
all: example.wasm example-js

%.wasm: %.wac
	wac encode -o $@ $<

example-js: example.wasm
	 jco transpile $< -o $@
