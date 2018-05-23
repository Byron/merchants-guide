#[macro_use]
extern crate failure;
extern crate regex;

use failure::{Error, ResultExt};
use std::{fmt, io::{BufRead, BufReader, Read, Write}, str::FromStr};

#[derive(Debug, Default)]
struct ConversionTable {}

impl ConversionTable {
    fn update(
        &mut self,
        tokens: impl Iterator<Item = Result<Token, Error>>,
    ) -> Result<Vec<Query>, Error> {
        let mut queries = Vec::new();
        for token in tokens {
            match token {
                Ok(token) => unimplemented!(),
                Err(err) => return Err(err),
            }
        }
        Ok(queries)
    }
}

enum Query {

}

enum Roman {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<Roman> for u32 {
    fn from(this: Roman) -> Self {
        use self::Roman::*;
        match this {
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
        }
    }
}

enum Token {
    RomanNumeralMapping { value: String, roman: Roman },
}

struct Answer<'a>(&'a Query, &'a ConversionTable);

impl<'a> fmt::Display for Answer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let re_set = regex::RegexSet::new(&[r"^\s+(?P<value>\w+)\s+is\s+(?P<roman>\w)$"]).unwrap();
        let matches = re_set.matches(s);
        match matches.len() {
            0 => bail!("Statement '{}' is unknown to us", s),
            1 => unimplemented!(),
            _ => bail!("Statement '{}' is ambiguous", s),
        }
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

pub fn answers(mut input: impl Read, mut output: impl Write) -> Result<(), Error> {
    let mut table = ConversionTable::default();
    let queries = table.update(parse(input))?;
    if queries.is_empty() {
        writeln!(output, "{:?}", table)?;
    } else {
        for query in queries {
            writeln!(output, "{}", Answer(&query, &table))?;
        }
    }
    Ok(())
}
