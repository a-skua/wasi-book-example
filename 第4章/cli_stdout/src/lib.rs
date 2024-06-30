#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::stdout::get_stdout;

struct Component;
impl Guest for Component {
    fn run() -> Result<(), ()> {
        let stdout = get_stdout();
        stdout.blocking_write_and_flush(b"Hello, world!\n").unwrap();
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
