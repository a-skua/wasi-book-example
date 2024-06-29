#[allow(warnings)]
mod bindings;

use bindings::example::hello::hello::greet;

fn main() {
    let greet = greet("asuka");
    println!("{greet}");
}
