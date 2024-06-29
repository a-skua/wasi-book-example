use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = args().nth(1).unwrap();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{contents}");
}
