.PHONY: all
all: compose.wasm

compose.wasm: hello/target/wasm32-wasi/release/hello.wasm component/target/wasm32-wasi/release/component.wasm
	wasm-tools compose -o $@ -d $^

hello/target/wasm32-wasi/release/hello.wasm:
	$(MAKE) -C hello

component/target/wasm32-wasi/release/component.wasm:
	$(MAKE) -C component
