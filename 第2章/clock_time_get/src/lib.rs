type Size = usize;
type Fd = usize;
type Errno = usize;
type Exitcode = usize;
type Clockid = usize;
type Timestamp = u64;

#[repr(C)]
pub struct Ciovec {
    pub buf: *const u8,
    pub buf_len: Size,
}

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    pub fn clock_time_get(id: Clockid, precision: Timestamp, time: *mut Timestamp) -> Errno;
    pub fn fd_write(fd: Fd, ciovs: *const Ciovec, ciovs_len: Size, written: *mut Size) -> Errno;
    pub fn proc_exit(rval: Exitcode);
}

#[no_mangle]
pub fn _start() {
    let mut time: Timestamp = 0;
    unsafe { clock_time_get(0, 0, &mut time) };

    // ナノ秒 --> 秒
    let time = (time / 1_000_000_000).to_string();

    let ciovs = [
        Ciovec {
            buf: time.as_ptr(),
            buf_len: time.len(),
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
