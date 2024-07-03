use wasi::{fd_write, Ciovec, FD_STDOUT};

#[export_name = "_start"]
fn main() {
    let message = b"Hello, world!\n";
    let ciovec = [Ciovec {
        buf: message.as_ptr(),
        buf_len: message.len(),
    }];

    unsafe { fd_write(FD_STDOUT, &ciovec).unwrap() };
}
