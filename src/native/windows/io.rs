unsafe extern "system" {
    #[link_name = "GetStdHandle"]
    unsafe fn get_std_handle(id: u32) -> usize;

    #[link_name = "GetConsoleOutputCP"]
    unsafe fn get_console_output_cp() -> u32;

    #[link_name = "SetConsoleOutputCP"]
    unsafe fn set_console_output_cp(code_page: u32) -> i32;

    #[link_name = "WriteConsoleA"]
    unsafe fn write_console(
        handle: usize,
        buf: *const u8,
        len: u32,
        written: *mut u32,
        reserved: usize,
    ) -> i32;
}

const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
const CP_UTF8: u32 = 65001;

pub fn print(s: &str) {
    unsafe {
        let code_page: u32 = get_console_output_cp();
        if code_page != CP_UTF8 {
            set_console_output_cp(CP_UTF8);
        }
        let handle: usize = get_std_handle(STD_OUTPUT_HANDLE);
        let mut written: u32 = 0;
        write_console(handle, s.as_ptr(), s.len() as u32, &mut written, 0);
    }
}
