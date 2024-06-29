.PHONY: all
all: hello.wasm wasm_exec.js

%.wasm: %.go
	GOARCH=wasm GOOS=js go build -o $@ $<

wasm_exec.js:
	cp "$(shell go env GOROOT)/misc/wasm/wasm_exec.js" $@
