// 実行時にインポートされるモジュールの定義
#[link(wasm_import_module = "env")]
extern "C" {
    fn write(ptr: *const u8, len: usize);
}

#[no_mangle]
pub fn _start() {
    let msg = b"Hello, world!";
    unsafe {
        write(msg.as_ptr(), msg.len());
    }
}
