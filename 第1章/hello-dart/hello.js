import { instantiate, invoke } from "./hello.mjs";

const filename = new URL("./hello.wasm", import.meta.url);

const instance = await instantiate(
  WebAssembly.compileStreaming(fetch(filename)),
);

invoke(instance);
