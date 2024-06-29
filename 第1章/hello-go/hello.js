import "./wasm_exec.js";

const go = new Go();

const filename = new URL("./hello.wasm", import.meta.url);

const { instance } = await WebAssembly.instantiateStreaming(
  fetch(filename),
  go.importObject,
);

go.run(instance);
