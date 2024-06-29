.PHONY: all
all: command.wasm reactor.wasm stdio.wasm stdio.exe env.wasm time.wasm random.wasm file.wasm

%.wasm: %.wat
	wasm-tools parse -o $@ $<

%.wasm: %.go
	GOARCH=wasm GOOS=wasip1 go build -o $@ $<

%.exe: %.c
	clang -o $@ $<
