use std::io::{stdin, stdout, Write};

fn main() {
    let _ = stdout().flush();
    let mut pattern = String::new();
    let mut path = String::new();
    stdin()
        .read_line(&mut pattern)
        .expect("Did not enter a correct string");

    stdin()
        .read_line(&mut path)
        .expect("Did not enter a correct string");

    println!("pattern: {}", pattern);
    println!("path: {}", path);
}
