#[macro_use]
extern crate failure;
extern crate failure_tools;
extern crate galactic_merchants_guide;

use failure_tools::ok_or_exit;
use failure::{Error, ResultExt};
use std::{env, fs::File, io::stdout};

fn run() -> Result<(), Error> {
    let filename = env::args().nth(1).ok_or_else(|| {
        format_err!(
            "USAGE: {} <input>\n\nWhere <input> is the input file with statements",
            env::args().next().expect("program name")
        )
    })?;
    let file_stream = File::open(&filename)
        .with_context(|_| format_err!("Could not open '{}' for reading", filename))?;

    let stdout = stdout();
    let lock = stdout.lock();
    galactic_merchants_guide::answers(file_stream, lock)
}

fn main() {
    ok_or_exit(run())
}
