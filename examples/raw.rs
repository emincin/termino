use termino::io::*;

fn main() {
    let ok: bool = enable_raw_mode();
    println!("Raw mode: {}", ok);
    while ok {
        let input: String = read_string(1024);
        if input.eq_ignore_ascii_case("q") {
            break;
        }
        print(&input);
    }
}
