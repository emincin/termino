unsafe extern "C" {
    unsafe fn read(fd: i32, buf: *mut u8, len: usize) -> isize;

    unsafe fn write(fd: i32, buf: *const u8, len: usize) -> isize;
}

const ERROR: isize = -1;
const STDIN_FILENO: i32 = 0;
const STDOUT_FILENO: i32 = 1;

pub fn print(s: &str) {
    unsafe {
        write(STDOUT_FILENO, s.as_ptr(), s.len());
    }
}

pub fn read_string(capacity: usize) -> String {
    unsafe {
        let mut s: String = String::with_capacity(capacity);
        let ret: isize = read(STDIN_FILENO, s.as_mut_ptr(), capacity);
        if ret == ERROR {
            return String::new();
        }
        s.as_mut_vec().set_len(ret as usize);
        return s;
    }
}
