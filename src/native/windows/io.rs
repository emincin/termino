unsafe extern "system" {
    #[link_name = "GetStdHandle"]
    unsafe fn get_std_handle(id: u32) -> usize;

    #[link_name = "GetConsoleMode"]
    unsafe fn get_console_mode(handle: usize, mode: *mut u32) -> i32;

    #[link_name = "SetConsoleMode"]
    unsafe fn set_console_mode(handle: usize, mode: u32) -> i32;

    #[link_name = "GetConsoleCP"]
    unsafe fn get_console_cp() -> u32;

    #[link_name = "SetConsoleCP"]
    unsafe fn set_console_cp(code_page: u32) -> i32;

    #[link_name = "GetConsoleOutputCP"]
    unsafe fn get_console_output_cp() -> u32;

    #[link_name = "SetConsoleOutputCP"]
    unsafe fn set_console_output_cp(code_page: u32) -> i32;

    #[link_name = "ReadConsoleA"]
    unsafe fn read_console(
        handle: usize,
        buf: *mut u8,
        len: u32,
        read: *mut u32,
        reserved: usize,
    ) -> i32;

    #[link_name = "WriteConsoleA"]
    unsafe fn write_console(
        handle: usize,
        buf: *const u8,
        len: u32,
        written: *mut u32,
        reserved: usize,
    ) -> i32;
}

const NULL: usize = 0;
const INVALID_HANDLE_VALUE: usize = -1i64 as usize;
const STD_INPUT_HANDLE: u32 = -10i32 as u32;
const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
const CP_UTF8: u32 = 65001;
const ENABLE_LINE_INPUT: u32 = 2;
const ENABLE_ECHO_INPUT: u32 = 4;

fn disable_input_flag(flag: u32) -> bool {
    unsafe {
        let handle: usize = get_std_handle(STD_INPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE || handle == NULL {
            return false;
        }
        let mut mode: u32 = 0;
        let ret: i32 = get_console_mode(handle, &mut mode);
        if ret == 0 {
            return false;
        }
        mode &= !flag;
        return set_console_mode(handle, mode) != 0;
    }
}

pub fn print(s: &str) {
    unsafe {
        let code_page: u32 = get_console_output_cp();
        if code_page != CP_UTF8 {
            set_console_output_cp(CP_UTF8);
        }
        let handle: usize = get_std_handle(STD_OUTPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE || handle == NULL {
            return;
        }
        let mut written: u32 = 0;
        write_console(handle, s.as_ptr(), s.len() as u32, &mut written, 0);
    }
}

pub fn read_string(capacity: usize) -> String {
    unsafe {
        let code_page: u32 = get_console_cp();
        if code_page != CP_UTF8 {
            set_console_cp(CP_UTF8);
        }
        let handle: usize = get_std_handle(STD_INPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE || handle == NULL {
            return String::new();
        }
        let mut s: String = String::with_capacity(capacity);
        let mut read: u32 = 0;
        let ret: i32 = read_console(handle, s.as_mut_ptr(), capacity as u32, &mut read, 0);
        if ret == 0 {
            return String::new();
        }
        s.as_mut_vec().set_len(read as usize);
        return s;
    }
}

pub fn disable_line_input() {
    disable_input_flag(ENABLE_LINE_INPUT);
}
