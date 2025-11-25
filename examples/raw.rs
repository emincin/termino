use termino::io::*;

fn main() {
    let ok: bool = enable_raw_mode();
    println!("Raw mode: {}", ok);
}
