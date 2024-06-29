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
    pub fn environ_get(environ: *mut usize, environ_buf: *mut u8) -> Errno;
    pub fn environ_sizes_get(environ_count: *mut Size, environ_buf_size: *mut Size) -> Errno;
    pub fn fd_write(fd: Fd, ciovs: *const Ciovec, ciovs_len: Size, written: *mut Size) -> Errno;
    pub fn proc_exit(rval: Exitcode);
}

#[no_mangle]
pub fn _start() {
    let mut environ_count: Size = 0;
    let mut environ_buf_size: Size = 0;
    unsafe { environ_sizes_get(&mut environ_count, &mut environ_buf_size) };

    let mut environ = vec![0; environ_count];
    let mut environ_buf = vec![0; environ_buf_size];
    unsafe { environ_get(environ.as_mut_ptr(), environ_buf.as_mut_ptr()) };

    for environ in environ {
        let environ = environ - environ_buf.as_ptr() as usize;
        let environ_buf = &environ_buf[environ..];
        let environ_buf = &environ_buf[..environ_buf.iter().position(|&byte| byte == 0).unwrap()];
        let environ = std::str::from_utf8(environ_buf).unwrap();
        let environ: Vec<&str> = environ.split('=').collect();
        let ciovs = [
            Ciovec {
                buf: environ[0].as_ptr(),
                buf_len: environ[0].len(),
            },
            Ciovec {
                buf: " = ".as_ptr(),
                buf_len: 3,
            },
            Ciovec {
                buf: environ[1].as_ptr(),
                buf_len: environ[1].len(),
            },
            Ciovec {
                buf: "\n".as_ptr(),
                buf_len: 1,
            },
        ];
        // unsafe { fd_write(1, ciovs.as_ptr(), ciovs.len(), &mut 0) };
        for i in 0..ciovs.len() {
            unsafe { fd_write(1, ciovs[i..].as_ptr(), 1, &mut 0) };
        }
    }

    unsafe { proc_exit(0) };
}
