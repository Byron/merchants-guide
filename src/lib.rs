extern crate failure;

use failure::{Error, ResultExt};
use std::io::{BufRead, BufReader, Read, Write};
use std::io::Cursor;
use std::str::FromStr;

#[derive(Debug)]
struct State {}

impl State {
    fn has_seen_queries(&self) -> bool {
        false
    }

    fn update(&mut self, tokens: impl Iterator<Item = Result<Token, Error>>) -> Result<(), Error> {
        unimplemented!()
    }
}

enum Token {
    RomanNumeralMapping { value: String, roman: String },
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        unimplemented!()
    }
}

fn parse(input: impl Read) -> impl Iterator<Item = Result<Token, Error>> {
    let mut input = BufReader::new(input);
    input.lines().map(|r| {
        r.context("Failed to read input")
            .map_err(|err| err.into())
            .and_then(|l| l.parse())
    })
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

pub fn answers(mut input: impl Read, mut output: impl Write) -> Result<(), Error> {
    let mut state = State::default();
    state.update(parse(input))?;
    if !state.has_seen_queries() {
        writeln!(output, "{:?}", state)?;
        return Ok(());
    }
    Ok(())
}
