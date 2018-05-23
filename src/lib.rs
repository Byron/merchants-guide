#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use failure::{Error, ResultExt};
use std::{fmt, io::{BufRead, BufReader, Read, Write}, str::FromStr};

lazy_static! {
    static ref RE_SET_ROMAN: regex::Regex =
        regex::Regex::new(r"^\s*(?P<value>\w+)\s+is\s+(?P<roman>\w)\s*$").expect("valid regex");
}

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

impl FromStr for Roman {
    type Err = Error;

    fn from_str(s: &str) -> Result<Roman, Error> {
        use self::Roman::*;
        Ok(match s {
            "I" => I,
            "V" => V,
            "X" => X,
            "L" => L,
            "C" => C,
            "D" => D,
            "M" => M,
            _ => return Err(format_err!("Invalid Roman numeral: '{}'", s)),
        })
    }
}

impl From<Roman> for u32 {
    fn from(this: Roman) -> u32 {
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
        match RE_SET_ROMAN.captures(s) {
            Some(captures) => Ok(Token::RomanNumeralMapping {
                value: captures["value"].to_owned(),
                roman: captures["roman"].parse()?,
            }),
            _ => Err(format_err!("'{}' could not be parsed", s)),
        }
    }
}

fn parse(input: impl Read) -> impl Iterator<Item = Result<Token, Error>> {
    let input = BufReader::new(input);
    input.lines().map(|r| {
        r.context("Failed to read at least one line from input")
            .map_err(Into::into)
            .and_then(|l| l.parse())
    })
}

pub fn answers(input: impl Read, mut output: impl Write) -> Result<(), Error> {
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
