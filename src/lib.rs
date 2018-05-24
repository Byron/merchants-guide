#![feature(slice_patterns)]

#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};
use std::{collections::BTreeMap, io::{BufRead, BufReader, Read, Write}, str::FromStr};

fn roman_to_decimal(romans: impl Iterator<Item = Roman>) -> Result<u32, Error> {
    let mut decimal = 0;
    let mut iter = romans.peekable();
    while let Some(c) = iter.next() {
        let c: u32 = c.into();
        let mut multiplier = 1;
        if let Some(&n) = iter.peek() {
            let n: u32 = n.into();
            if n > c {
                multiplier = -1;
            }
        }
        decimal += i64::from(c) * multiplier;
    }
    if decimal == 0 {
        bail!("No romans literals provided")
    } else if decimal < 0 {
        bail!("Converted romans into negative decimal value {}", decimal);
    }
    Ok(decimal as u32)
}

fn symbols_to_romans(
    symbol_to_romans: &BTreeMap<String, Roman>,
    symbols: impl Iterator<Item = impl AsRef<str>>,
) -> Result<Vec<Roman>, Error> {
    symbols
        .map(|s| {
            symbol_to_romans.get(s.as_ref()).cloned().ok_or_else(|| {
                format_err!("No roman value was associated with symbol '{}'", s.as_ref())
            })
        })
        .collect()
}

fn symbols_to_decimal(
    symbol_to_romans: &BTreeMap<String, Roman>,
    symbols: impl Iterator<Item = impl AsRef<str>>,
) -> Result<u32, Error> {
    roman_to_decimal(symbols_to_romans(symbol_to_romans, symbols)?.into_iter())
}

enum Query<'a> {
    Other,
    Roman {
        symbols: Vec<String>,
    },
    Product {
        symbols: Vec<String>,
        product: &'a str,
    },
}

impl<'a> Query<'a> {
    fn answer(
        &self,
        symbol_to_romans: &BTreeMap<String, Roman>,
        product_prices: &BTreeMap<String, f32>,
    ) -> Result<String, Error> {
        use self::Query::*;
        Ok(match self {
            Other => String::from("I have no idea what you are talking about"),
            Product { symbols, product } => {
                let single_product_price = product_prices.get(product.to_owned()).ok_or_else(
                    || format_err!("Product named '{}' was not yet encountered", product),
                )?;
                let decimal_multiplier = symbols_to_decimal(symbol_to_romans, symbols.iter())?;
                let product_price = decimal_multiplier as f32 * single_product_price;
                format!(
                    "{} {} is {} Credits",
                    symbols.join(" "),
                    product,
                    product_price
                )
            }
            Roman { symbols } => {
                let decimal_value = symbols_to_decimal(symbol_to_romans, symbols.iter())?;
                format!("{} is {}", symbols.join(" "), decimal_value)
            }
        })
    }
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
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

enum Token<'a> {
    RomanNumeralMapping {
        symbol: &'a str,
        roman: Roman,
    },
    PriceAssignment {
        credits: f32,
        product: &'a str,
        symbols: Vec<String>,
    },
    Other(Query<'a>),
}

fn token_from_str<'a>(s: &'a str) -> Result<Token<'a>, Error> {
    use self::Token::*;
    fn to_owned(s: &[&str]) -> Vec<String> {
        s.iter().map(|&s| String::from(s)).collect()
    }
    let tokens: Vec<_> = s.split_whitespace().collect();
    Ok(match *tokens.as_slice() {
        [symbol, "is", roman] => RomanNumeralMapping {
            symbol,
            roman: roman.parse()?,
        },
        [ref symbols.., product, "is", credits, "Credits"] => PriceAssignment {
            credits: credits.parse::<f32>().with_context(|_| {
                format!("Could not parse floating point number from '{}'", credits)
            })?,
            product,
            symbols: to_owned(symbols),
        },
        ["how", "much", "is", ref symbols.., "?"] => Other(Query::Roman {
            symbols: to_owned(symbols),
        }),
        ["how", "many", "Credits", "is", ref symbols.., product, "?"] => Other(Query::Product {
            symbols: to_owned(symbols),
            product,
        }),
        ["how", "much", _.., "?"] => return Ok(Other(Query::Other)),
        _ => {
            return Err(format_err!("'{}' could not be parsed", s));
        }
    })
}

pub fn answers(input: impl Read, mut output: impl Write) -> Result<(), Error> {
    use self::Token::*;
    let mut symbol_to_romans: BTreeMap<String, Roman> = BTreeMap::new();
    let mut product_prices: BTreeMap<String, f32> = BTreeMap::new();
    let input = BufReader::new(input);

    for line in input.lines() {
        let line = line.context("Failed to read at least one line from input")?;
        let token = token_from_str(&line);
        match token {
            Ok(token) => match token {
                Other(query) => writeln!(
                    output,
                    "{}",
                    query.answer(&symbol_to_romans, &product_prices)?
                )?,
                RomanNumeralMapping { symbol, roman } => {
                    symbol_to_romans.insert(symbol.to_owned(), roman);
                }
                PriceAssignment {
                    credits,
                    product,
                    symbols,
                } => {
                    let product_price =
                        credits / symbols_to_decimal(&symbol_to_romans, symbols.iter())? as f32;
                    product_prices.insert(product.to_owned(), product_price);
                }
            },
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
