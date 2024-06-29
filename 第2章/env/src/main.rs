use std::env::{args, var};

fn main() {
    // 環境変数
    let example = var("EXAMPLE").unwrap_or("".to_string());
    println!("EXAMPLE = {example}");

    // 引数
    let args: Vec<String> = args().collect();
    println!("ARGS = {args:?}");
}
