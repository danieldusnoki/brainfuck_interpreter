use std::io;

fn main() {
    println!("Type in the brainfuck code to interpret!");

    let mut code = String::new();
    io::stdin().read_line(&mut code)
        .expect("Failed to read line");

    
}
