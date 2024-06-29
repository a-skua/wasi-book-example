.PHONY: all
all: compose.wasm

compose.wasm: hello.wasm component.wasm
	wasm-tools compose -o $@ -d $^

%.wasm: %.js %.wit
	jco componentize $< --wit $(word 2,$^) -o $@

component.wasm: component/target/wasm32-wasi/release/component.wasm
	cp $< $@

component/target/wasm32-wasi/release/component.wasm:
	$(MAKE) -C component
