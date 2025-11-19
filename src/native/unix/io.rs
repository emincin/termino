unsafe extern "C" {
    unsafe fn write(fd: i32, buf: *const u8, len: usize) -> isize;
}

const STDOUT_FILENO: i32 = 1;

pub fn print(s: &str) {
    unsafe {
        write(STDOUT_FILENO, s.as_ptr(), s.len());
    }
}
