#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::stdin::get_stdin;
use bindings::wasi::cli::stdout::get_stdout;

struct Component;
impl Guest for Component {
    fn run() -> Result<(), ()> {
        let stdout = get_stdout();
        stdout.blocking_write_and_flush(b"Your name: ").unwrap();

        let stdin = get_stdin();
        let name = stdin.blocking_read(1024).unwrap();
        let name = String::from_utf8(name).unwrap();
        let name = name.trim();

        stdout
            .blocking_write_and_flush(format!("Hello, {name}!\n").as_bytes())
            .unwrap();
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
