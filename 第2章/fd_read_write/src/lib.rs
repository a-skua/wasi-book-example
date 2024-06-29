type Size = usize;
type Fd = usize;
type Errno = usize;
type Exitcode = usize;

#[repr(C)]
pub struct Iovec {
    pub buf: *mut u8,
    pub buf_len: Size,
}

#[repr(C)]
pub struct Ciovec {
    pub buf: *const u8,
    pub buf_len: Size,
}

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    pub fn fd_read(fd: Fd, iovs: *const Iovec, iovs_len: Size, read: *mut Size) -> Errno;
    pub fn fd_write(fd: Fd, ciovs: *const Ciovec, ciovs_len: Size, written: *mut Size) -> Errno;
    pub fn proc_exit(rval: Exitcode);
}

#[no_mangle]
pub fn _start() {
    let mut nwritten: Size = 0;
    let mut nread: Size = 0;
    let mut buf = [0; 100];

    let ciovs = [Ciovec {
        buf: "Your name: ".as_ptr(),
        buf_len: 11,
    }];
    unsafe { fd_write(1, ciovs.as_ptr(), 1, &mut nwritten) };

    // 標準入力
    let iovs = [Iovec {
        buf: buf.as_mut_ptr(),
        buf_len: buf.len(),
    }];
    unsafe { fd_read(0, iovs.as_ptr(), 1, &mut nread) };

    // 標準出力
    let ciovs = [
        Ciovec {
            buf: "Hello, ".as_ptr(),
            buf_len: 7,
        },
        Ciovec {
            buf: buf.as_ptr(),
            buf_len: nread - 1, // 改行を含めない
        },
        Ciovec {
            buf: "!\n".as_ptr(),
            buf_len: 2,
        },
    ];
    // unsafe { fd_write(1, ciovs.as_ptr(), ciovs.len(), &mut nwritten) };
    for i in 0..ciovs.len() {
        unsafe { fd_write(1, ciovs[i..].as_ptr(), 1, &mut nwritten) };
    }

    unsafe { proc_exit(0) };
}
