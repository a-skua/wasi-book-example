// ファイル名
const filename = new URL("example.wasm", import.meta.url);

// バイナリを取得
const binary = await fetch(filename).then((res) => res.arrayBuffer());

// モジュールを作成
const module = new WebAssembly.Module(binary);

// インスタンスを作成
const instance = new WebAssembly.Instance(module);

// 実行
console.log(instance.exports.add(1, 2)); // 3
