#[allow(warnings)]
mod bindings;

use crate::bindings::exports::example::hello::hello::Guest;

struct Component;

impl Guest for Component {
    fn greet(name: String) -> String {
        format!("Hello, {name}!")
    }
}

bindings::export!(Component with_types_in bindings);
