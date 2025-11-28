#[derive(Debug, Default)]
#[repr(C)]
struct termios {
    c_iflag: u64,
    c_oflag: u64,
    c_cflag: u64,
    c_lflag: u64,
    c_cc: [u8; 20],
    c_ispeed: u64,
    c_ospeed: u64,
}

unsafe extern "C" {
    unsafe fn isatty(fd: i32) -> i32;

    unsafe fn read(fd: i32, buf: *mut u8, len: usize) -> isize;

    unsafe fn write(fd: i32, buf: *const u8, len: usize) -> isize;

    unsafe fn tcgetattr(fd: i32, term: *mut termios) -> i32;

    unsafe fn tcsetattr(fd: i32, act: i32, term: *const termios) -> i32;
}

const OK: i32 = 0;
const ERROR: isize = -1;
const STDIN_FILENO: i32 = 0;
const STDOUT_FILENO: i32 = 1;
const TCSANOW: i32 = 0;
const TCSADRAIN: i32 = 1;
const TCSAFLUSH: i32 = 2;

pub fn print(s: &str) -> usize {
    unsafe {
        if isatty(STDOUT_FILENO) == 0 {
            return 0;
        }
        let ret: isize = write(STDOUT_FILENO, s.as_ptr(), s.len());
        if ret < 0 {
            if ret == ERROR {}
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
        if ret < 0 {
            if ret == ERROR {}
            return String::new();
        }
        s.as_mut_vec().set_len(ret as usize);
        return s;
    }
}

pub fn enable_raw_mode() -> bool {
    unsafe {
        let mut term: termios = termios::default();
        let ret: i32 = tcgetattr(STDIN_FILENO, &mut term);
        if ret != OK {
            return false;
        }
        let ret: i32 = tcsetattr(STDIN_FILENO, TCSANOW, &term);
        if ret != OK {
            return false;
        }
        return true;
    }
}

pub fn disable_raw_mode() -> bool {
    unsafe {
        return true;
    }
}
