#![feature(slice_patterns)]

#[macro_use]
extern crate failure;
extern crate lines;

use lines::linereader::LineReader;
use failure::{Error, ResultExt};
use std::{collections::BTreeMap, io::{Read, Write}, str::{FromStr, from_utf8}};

pub fn answers(input: impl Read, mut output: impl Write) -> Result<(), Error> {
    let mut symbol_to_romans: BTreeMap<String, Roman> = BTreeMap::new();
    let mut product_prices: BTreeMap<String, f32> = BTreeMap::new();
    let mut reader = LineReader::new(input);

    loop {
        let line = reader
            .read_line()
            .context("Failed to read at least one line from input")?;
        if line.is_empty() {
            return Ok(());
        }
        let line = &line[0..line.len() - 1];
        let mut tokens = Vec::with_capacity(10);
        line.split(|&b| b == b' ').fold(&mut tokens, |acc, t| {
            acc.push(t);
            acc
        });
        match *tokens {
            [symbol, b"is", roman] => {
                if !symbol_to_romans.contains_key(from_utf8(symbol)?) {
                    symbol_to_romans
                        .insert(from_utf8(symbol)?.to_owned(), from_utf8(roman)?.parse()?);
                }
            }
            [ref symbols.., product, b"is", credits, b"Credits"] => {
                let credits = from_utf8(credits)?;
                let credits = credits.parse::<f32>().with_context(|_| {
                    format!("Could not parse floating point number from '{}'", credits)
                })?;
                let product = from_utf8(product)?;
                if !product_prices.contains_key(product) {
                    let product_price = credits
                        / symbols_to_decimal(
                            &symbol_to_romans,
                            symbols.iter().filter_map(|b| from_utf8(b).ok()),
                        )? as f32;
                    product_prices.insert(product.to_owned(), product_price);
                }
            }
            [b"how", b"much", b"is", ref symbols.., b"?"] => {
                let decimal_value = symbols_to_decimal(
                    &symbol_to_romans,
                    symbols.iter().filter_map(|b| from_utf8(b).ok()),
                )?;
                writeln!(
                    output,
                    "{} is {}",
                    symbols
                        .iter()
                        .filter_map(|b| from_utf8(b).ok())
                        .collect::<Vec<_>>()
                        .join(" "),
                    decimal_value
                )?;
            }
            [b"how", b"many", b"Credits", b"is", ref symbols.., product, b"?"] => {
                let product = from_utf8(product)?;
                let single_product_price = product_prices.get(product).ok_or_else(|| {
                    format_err!("Product named '{}' was not yet encountered", product)
                })?;
                let decimal_multiplier = symbols_to_decimal(
                    &symbol_to_romans,
                    symbols.iter().filter_map(|b| from_utf8(b).ok()),
                )?;
                let product_price = decimal_multiplier as f32 * single_product_price;
                writeln!(
                    output,
                    "{} {} is {} Credits",
                    symbols
                        .iter()
                        .filter_map(|b| from_utf8(b).ok())
                        .collect::<Vec<_>>()
                        .join(" "),
                    product,
                    product_price
                )?;
            }
            [b"how", b"much", _.., b"?"] => {
                writeln!(output, "I have no idea what you are talking about")?;
            }
            _ => {
                return Err(format_err!("'{}' could not be parsed", from_utf8(line)?));
            }
        }
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
    symbol_table: &BTreeMap<String, Roman>,
    symbols: impl Iterator<Item = impl AsRef<str>>,
) -> Result<Vec<Roman>, Error> {
    symbols
        .map(|s| {
            symbol_table.get(s.as_ref()).cloned().ok_or_else(|| {
                format_err!("No roman value was associated with symbol '{}'", s.as_ref())
            })
        })
        .collect()
}

fn symbols_to_decimal(
    symbol_table: &BTreeMap<String, Roman>,
    symbols: impl Iterator<Item = impl AsRef<str>>,
) -> Result<u32, Error> {
    roman_to_decimal(symbols_to_romans(symbol_table, symbols)?.into_iter())
}
