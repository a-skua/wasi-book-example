const filename = new URL(
  "target/wasm32-unknown-unknown/release/hello.wasm",
  import.meta.url,
);

// WebAssemblyの代わりに標準出力に出力する関数
function write(ptr, len) {
  console.log(new TextDecoder().decode(
    new Uint8Array(memory.buffer, ptr, len),
  ));
}

const { instance } = await WebAssembly.instantiateStreaming(
  fetch(filename),
  // モジュールのインポート
  { env: { write } },
);

const memory = instance.exports.memory;
instance.exports._start();
