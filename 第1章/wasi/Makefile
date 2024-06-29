.PHONY: all
all: hello.wasm

%.wasm: %.go
	GOARCH=wasm GOOS=wasip1 go build -o $@ $<
