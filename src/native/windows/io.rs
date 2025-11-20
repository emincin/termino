unsafe extern "system" {
    #[link_name = "GetStdHandle"]
    unsafe fn get_std_handle(id: u32) -> usize;

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

pub fn print(s: &str) {
    unsafe {
        let handle: usize = get_std_handle(STD_OUTPUT_HANDLE);
        let mut written: u32 = 0;
        write_console(handle, s.as_ptr(), s.len() as u32, &mut written, 0);
    }
}
