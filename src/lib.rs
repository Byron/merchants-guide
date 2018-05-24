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
    static ref RE_ASSIGN_CREDITS: regex::Regex = regex::Regex::new(
        r"^\s*(?P<values>[\w\s]+)\s+(?P<product>\w+)\s+is\s+(?P<credits>\d+)\s+[cC]redits$"
    ).expect("valid regex");
}

fn roman_value(romans: &[Roman]) -> Result<u32, Error> {
    Ok(match romans.len() {
        0 => bail!("Cannot compute value for missing roman value"),
        1 => romans[0].into(),
        _ => {
            let mut value = 0;
            let mut iter = romans.iter().peekable();
            while let Some(&c) = iter.next() {
                let c: u32 = c.into();
                let mut multiplier = 1;
                if let Some(&&n) = iter.peek() {
                    let n: u32 = n.into();
                    if n > c {
                        multiplier = -1;
                    }
                }
                value += c as i64 * multiplier;
            }
            if value < 0 {
                bail!("Converted '{:?}' into negative value {}", romans, value);
            }
            value as u32
        }
    })
}

#[derive(Debug, Default)]
struct ConversionTable {
    value_to_romans: Vec<(String, Roman)>,
    product_prices: Vec<(String, u32)>,
}

impl ConversionTable {
    fn translate_values_to_romans(
        &self,
        values_space_separated: &str,
    ) -> Result<Vec<Roman>, Error> {
        values_space_separated
            .split_whitespace()
            .map(|t| {
                self.value_to_romans
                    .iter()
                    .find(|(v, _)| v == t)
                    .map(|(_, r)| r.to_owned())
                    .ok_or_else(|| format_err!("No roman value was associated with '{}'", t))
            })
            .collect()
    }


    fn update(
        &mut self,
        tokens: impl Iterator<Item = Result<Token, Error>>,
    ) -> Result<Vec<Query>, Error> {
        use self::Token::*;
        let queries = Vec::new();
        for token in tokens {
            match token {
                Ok(token) => match token {
                    RomanNumeralMapping { value, roman } => {
                        self.value_to_romans.push((value, roman));
                    }
                    PriceAssignment {
                        credits,
                        product,
                        values_space_separated,
                    } => {
                        let product_price = credits
                            .checked_div(roman_value(&self.translate_values_to_romans(
                                &values_space_separated,
                            )?)?)
                            .ok_or_else(|| {
                                format_err!(
                                    "The roman value corresponding to '{}' was invalid.",
                                    values_space_separated
                                )
                            })?;
                        self.product_prices.push((product, product_price));
                    }
                },
                Err(err) => return Err(err),
            }
        }
        Ok(queries)
    }
}

enum Query {

}

#[derive(Debug, Hash, Clone, Copy)]
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
    RomanNumeralMapping {
        value: String,
        roman: Roman,
    },
    PriceAssignment {
        credits: u32,
        product: String,
        values_space_separated: String,
    },
}

struct Answer<'a>(&'a Query, &'a ConversionTable);

impl<'a> fmt::Display for Answer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Token, Error> {
        Ok(
            match (RE_SET_ROMAN.captures(s), RE_ASSIGN_CREDITS.captures(s)) {
                (Some(captures), None) => Token::RomanNumeralMapping {
                    value: captures["value"].to_owned(),
                    roman: captures["roman"].parse()?,
                },
                (None, Some(captures)) => Token::PriceAssignment {
                    credits: captures["credits"].parse::<u32>().with_context(|_| {
                        format!(
                            "Could not obtain unsigned integer from '{}'",
                            &captures["credits"]
                        )
                    })?,
                    product: captures["product"].to_owned(),
                    values_space_separated: captures["values"].to_owned(),
                },
                _ => return Err(format_err!("'{}' could not be parsed", s)),
            },
        )
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
