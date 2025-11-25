use termino::info::hello_termino;
use termino::io::print;
use termino::io::read_string;

fn main() {
    println!("Test 1");
    hello_termino();
    print("print!\n");
    print("😄\n");
    print("Input something: ");
    let input: String = read_string(16);
    print("Your input: ");
    print(&input);
}
