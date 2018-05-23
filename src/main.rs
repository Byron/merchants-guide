#[macro_use]
extern crate failure;
extern crate galactic_merchants_guide;

use failure::{Error, ResultExt};
use std::{env, process, fs::File, io::{stderr, stdout, Write}};

// TODO: put this into a crate, like 'failure-print' or 'failure-chain'
pub fn print_causes<E, W>(e: E, mut w: W)
where
    E: Into<Error>,
    W: Write,
{
    let e = e.into();
    let causes = e.causes().collect::<Vec<_>>();
    let num_causes = causes.len();
    for (index, cause) in causes.iter().enumerate() {
        if index == 0 {
            writeln!(w, "{}", cause).ok();
            if num_causes > 1 {
                writeln!(w, "Caused by: ").ok();
            }
        } else {
            writeln!(w, " {}: {}", num_causes - index, cause).ok();
        }
    }
}

pub fn ok_or_exit<T, E>(r: Result<T, E>) -> T
where
    E: Into<Error>,
{
    match r {
        Ok(r) => r,
        Err(e) => {
            stdout().flush().ok();
            write!(stderr(), "error: ").ok();
            print_causes(e, stderr());
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    let filename = env::args().nth(1).ok_or_else(|| {
        format_err!(
            "USAGE: {} <input>\n\nWhere <input> is the input file with statements",
            env::args().next().expect("program name")
        )
    })?;
    let file_stream = File::open(&filename)
        .with_context(|_| format_err!("Could not open '{}' for reading", filename))?;

    galactic_merchants_guide::answers(file_stream, stdout())
}

fn main() {
    ok_or_exit(run())
}
