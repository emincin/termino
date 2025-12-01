use termino::io::*;

fn main() {
    let ok: bool = enable_raw_mode();
    println!("Raw mode: {}", ok);
    while ok {
        let input: String = read_string(1024);
        if input.is_empty() {
            continue;
        }
        if input.len() == 1 && input.bytes().all(|b: u8| b == 27) {
            break;
        }
        print!("{:?} => ", input.as_bytes());
        for (i, c) in input.chars().enumerate() {
            if i != input.chars().count() - 1 {
                print!("{}, ", c as u32);
            } else {
                println!("{}", c as u32);
            }
        }
    }
}
