#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::environment::get_arguments;
use bindings::wasi::cli::stdout::get_stdout;
use bindings::wasi::filesystem::preopens::get_directories;
use bindings::wasi::filesystem::types::{DescriptorFlags, OpenFlags, PathFlags};

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let (dir, _) = &get_directories()[0];

        // Open file
        let filename = &get_arguments()[1];
        let file = dir
            .open_at(
                PathFlags::empty(),
                filename,
                OpenFlags::empty(),
                DescriptorFlags::READ,
            )
            .unwrap();

        // Read file
        let (bin, _) = file.read(1024, 0).unwrap();
        let stdout = get_stdout();
        stdout.blocking_write_and_flush(&bin).unwrap();
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
