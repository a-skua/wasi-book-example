type Size = usize;
type Fd = usize;
type Errno = usize;
type Exitcode = usize;

#[repr(C)]
pub struct Ciovec {
    pub buf: *const u8,
    pub buf_len: Size,
}

#[repr(C)]
pub struct PrestatDir {
    pub pr_name_len: Size,
}

#[repr(C)]
pub struct Prestat {
    pub tag_size: u8,
    _pad: [u8; 3],
    pub prestat_dir: PrestatDir,
}

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    pub fn fd_prestat_get(fd: Fd, prestat: *mut Prestat) -> Errno;
    pub fn fd_prestat_dir_name(fd: Fd, path: *mut u8, path_len: Size) -> Errno;
    pub fn fd_write(fd: Fd, ciovs: *const Ciovec, ciovs_len: Size, written: *mut Size) -> Errno;
    pub fn proc_exit(rval: Exitcode);
}

#[no_mangle]
pub fn _start() {
    let mut fd = 3;
    let mut prestat = Prestat {
        tag_size: 0,
        _pad: [0; 3],
        prestat_dir: PrestatDir { pr_name_len: 0 },
    };

    while let 0 = unsafe { fd_prestat_get(fd, &mut prestat) } {
        let mut path = vec![0; prestat.prestat_dir.pr_name_len];
        unsafe { fd_prestat_dir_name(fd, path.as_mut_ptr(), path.len()) };

        let ciovs = [
            Ciovec {
                buf: "preopened: ".as_ptr(),
                buf_len: 11,
            },
            Ciovec {
                buf: path.as_ptr(),
                buf_len: path.len(),
            },
            Ciovec {
                buf: "\n".as_ptr(),
                buf_len: 1,
            },
        ];
        unsafe { fd_write(1, ciovs.as_ptr(), ciovs.len(), &mut 0) };
        for i in 0..ciovs.len() {
            unsafe { fd_write(1, ciovs.as_ptr().add(i), 1, &mut 0) };
        }
        fd += 1;
    }

    unsafe { proc_exit(0) };
}
