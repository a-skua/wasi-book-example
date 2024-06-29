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
    pub fn sock_accept(fd: Fd, fdflags: u16, new_fd: *mut Fd) -> Errno;
    pub fn sock_send(
        fd: Fd,
        ciovs: *const Ciovec,
        ciovs_len: Size,
        si_flags: u16,
        written: *mut Size,
    ) -> Errno;
    pub fn sock_shutdown(fd: Fd, how: u16) -> Errno;
    pub fn fd_write(fd: Fd, ciovs: *const Ciovec, ciovs_len: Size, nwritten: *mut Size) -> Errno;
    pub fn proc_exit(rval: Exitcode);
}

const ERRNO_AGAIN: Errno = 6;
const HTTP_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 18\r\n\r\nHello, WASI 0.1!\r\n";

#[no_mangle]
pub fn _start() {
    let mut fd = 0;
    while let ERRNO_AGAIN = unsafe { sock_accept(3, 0, &mut fd) } {}

    let ciovs = [Ciovec {
        buf: HTTP_RESPONSE.as_ptr(),
        buf_len: HTTP_RESPONSE.len(),
    }];
    unsafe { sock_send(fd, ciovs.as_ptr(), ciovs.len(), 0, &mut 0) };

    unsafe { sock_shutdown(fd, 3) };

    unsafe { proc_exit(0) };
}
