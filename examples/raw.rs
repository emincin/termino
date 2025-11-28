use termino::io::*;
use termino::utils::*;

fn main() {
    let ok: bool = enable_raw_mode();
    println!("Raw mode: {}", ok);
    while ok {
        let input: String = read_string(1024);
        if input.eq_ignore_ascii_case("q") {
            break;
        }
        if input.len() == 1 && is_all_control(&input) {
            for c in input.chars() {
                let s: String = (c as u32).to_string();
                print(&s);
            }
        }
        print(&input);
    }
}
