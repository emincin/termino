unsafe extern "C" {
    unsafe fn isatty(fd: i32) -> i32;

    unsafe fn read(fd: i32, buf: *mut u8, len: usize) -> isize;

    unsafe fn write(fd: i32, buf: *const u8, len: usize) -> isize;
}

const ERROR: isize = -1;
const STDIN_FILENO: i32 = 0;
const STDOUT_FILENO: i32 = 1;

pub fn print(s: &str) -> usize {
    unsafe {
        if isatty(STDOUT_FILENO) == 0 {
            return 0;
        }
        let ret: isize = write(STDOUT_FILENO, s.as_ptr(), s.len());
        if ret < 0 {
            return 0;
        }
        return ret as usize;
    }
}

pub fn read_string(capacity: usize) -> String {
    unsafe {
        if isatty(STDIN_FILENO) == 0 {
            return String::new();
        }
        let mut s: String = String::with_capacity(capacity);
        let ret: isize = read(STDIN_FILENO, s.as_mut_ptr(), capacity);
        if ret == ERROR {
            return String::new();
        }
        s.as_mut_vec().set_len(ret as usize);
        return s;
    }
}
