#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::environment::get_environment;
use bindings::wasi::cli::stdout::get_stdout;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let env = get_environment();
        let stdout = get_stdout();
        for (name, value) in env {
            stdout
                .blocking_write_and_flush(format!("{name}={value}\n").as_bytes())
                .unwrap();
        }
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
