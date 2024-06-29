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
    pub fn args_get(argv: *mut usize, argv_buf: *mut u8) -> Errno;
    pub fn args_sizes_get(argv_count: *mut Size, argv_buf_size: *mut Size) -> Errno;
    pub fn fd_write(fd: Fd, ciovs: *const Ciovec, ciovs_len: Size, written: *mut Size) -> Errno;
    pub fn proc_exit(rval: Exitcode);
}

#[no_mangle]
pub fn _start() {
    let mut argv_count: Size = 0;
    let mut argv_buf_size: Size = 0;
    unsafe { args_sizes_get(&mut argv_count, &mut argv_buf_size) };

    let mut argv = vec![0; argv_count];
    let mut argv_buf = vec![0; argv_buf_size];
    unsafe { args_get(argv.as_mut_ptr(), argv_buf.as_mut_ptr()) };

    for i in 0..argv.len() {
        let argv = argv[i];
        let argv = argv - argv_buf.as_ptr() as usize;
        let argv_buf = &argv_buf[argv..];
        let argv_buf = &argv_buf[..argv_buf.iter().position(|&byte| byte == 0).unwrap()];

        let i = i.to_string();
        let ciovs = [
            Ciovec {
                buf: "ARGS[".as_ptr(),
                buf_len: 5,
            },
            Ciovec {
                buf: i.as_ptr(),
                buf_len: i.len(),
            },
            Ciovec {
                buf: "] = ".as_ptr(),
                buf_len: 4,
            },
            Ciovec {
                buf: argv_buf.as_ptr(),
                buf_len: argv_buf.len(),
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
