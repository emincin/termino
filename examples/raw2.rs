use termino::io::*;

fn main() {
    let ok: bool = enable_raw_mode();
    println!("Raw mode: {}", ok);
    while ok {
        let input: String = read_string(1024);
    }
}
