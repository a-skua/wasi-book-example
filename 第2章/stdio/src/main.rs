use std::io::{stdin, stdout, Write};

fn main() {
    print!("Your name: ");
    stdout().flush().unwrap();

    // 標準入力
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    // 標準出力
    println!("Hello, {name}!");
}
