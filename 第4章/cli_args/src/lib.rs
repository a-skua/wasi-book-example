#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::environment::get_arguments;
use bindings::wasi::cli::stdout::get_stdout;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let args = get_arguments();
        let stdout = get_stdout();
        for arg in args {
            stdout
                .blocking_write_and_flush(format!("{arg}\n").as_bytes())
                .unwrap();
        }
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
