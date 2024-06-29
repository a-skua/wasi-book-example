type Size = usize;
type Fd = usize;
type Errno = usize;
type Exitcode = usize;

#[repr(C)]
pub struct Ciovec {
    pub buf: *const u8,
    pub buf_len: Size,
}

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    pub fn random_get(buf: *mut u8, buf_len: Size) -> Errno;
    pub fn fd_write(fd: Fd, ciovs: *const Ciovec, ciovs_len: Size, written: *mut Size) -> Errno;
    pub fn proc_exit(rval: Exitcode);
}

#[no_mangle]
pub fn _start() {
    let mut buf = [0; 8];
    unsafe { random_get(buf.as_mut_ptr(), buf.len()) };

    let num = u64::from_ne_bytes(buf).to_string();
    let ciovs = [
        Ciovec {
            buf: num.as_ptr(),
            buf_len: num.len(),
        },
        Ciovec {
            buf: "\n".as_ptr(),
            buf_len: 1,
        },
    ];
    // unsafe { fd_write(1, ciovs.as_ptr(), ciovs.len(), &mut 0) };
    for i in 0..ciovs.len() {
        unsafe { fd_write(1, ciovs.as_ptr().add(i), 1, &mut 0) };
    }

    unsafe { proc_exit(0) };
}
