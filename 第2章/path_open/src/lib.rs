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
    pub fn path_open(
        fd: Fd,
        dirflags: u32,
        path: *const u8,
        path_len: Size,
        oflags: u16,
        fs_rights_base: u64,
        fs_rights_inheriting: u64,
        fdflags: u16,
        new_fd: *mut Fd,
    ) -> Errno;
    pub fn fd_read(fd: Fd, iovs: *const Iovec, iovs_len: Size, nread: *mut Size) -> Errno;
    pub fn fd_write(fd: Fd, ciovs: *const Ciovec, ciovs_len: Size, written: *mut Size) -> Errno;
    pub fn args_get(argv: *mut usize, argv_buf: *mut u8) -> Errno;
    pub fn args_sizes_get(argv_count: *mut Size, argv_buf_size: *mut Size) -> Errno;
    pub fn proc_exit(rval: Exitcode);
}

#[no_mangle]
pub fn _start() {
    let fd = 3;
    let mut prestat = Prestat {
        tag_size: 0,
        _pad: [0; 3],
        prestat_dir: PrestatDir { pr_name_len: 0 },
    };
    let errno = unsafe { fd_prestat_get(fd, &mut prestat) };
    if errno > 0 {
        panic!("fd_prestat_get failed: {errno}");
    }

    // 引数からファイル名を取得
    let mut argv_count: Size = 0;
    let mut argv_buf_size: Size = 0;
    unsafe { args_sizes_get(&mut argv_count, &mut argv_buf_size) };

    let mut argv = vec![0; argv_count];
    let mut argv_buf = vec![0; argv_buf_size];
    unsafe { args_get(argv.as_mut_ptr(), argv_buf.as_mut_ptr()) };

    let argv = argv[1];
    let argv = argv - argv_buf.as_ptr() as usize;
    let argv_buf = &argv_buf[argv..];
    let argv_buf = &argv_buf[..argv_buf.iter().position(|&byte| byte == 0).unwrap()];
    let path = argv_buf;

    // ファイルを開く
    let mut new_fd = 0;
    let errno = unsafe { path_open(fd, 0, path.as_ptr(), path.len(), 0, 0, 0, 0, &mut new_fd) };
    if errno > 0 {
        panic!("path_open failed: {errno}");
    }

    // ファイルを読み込む
    let mut nread: Size = 0;
    let mut buf = [0; 1024];
    let iovs = [Iovec {
        buf: buf.as_mut_ptr(),
        buf_len: buf.len(),
    }];
    unsafe { fd_read(new_fd, iovs.as_ptr(), iovs.len(), &mut nread) };

    // 標準出力
    let ciovs = [Ciovec {
        buf: buf.as_ptr(),
        buf_len: nread,
    }];
    unsafe { fd_write(1, ciovs.as_ptr(), ciovs.len(), &mut 0) };

    unsafe { proc_exit(0) };
}
