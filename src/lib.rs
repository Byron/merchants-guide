#![feature(slice_patterns)]

#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};
use std::{collections::BTreeMap, io::{BufRead, BufReader, Read, Write}, str::FromStr};

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

pub fn answers(input: impl Read, mut output: impl Write) -> Result<(), Error> {
    let mut symbol_to_romans: BTreeMap<String, Roman> = BTreeMap::new();
    let mut product_prices: BTreeMap<String, f32> = BTreeMap::new();
    let input = BufReader::new(input);

    for line in input.lines() {
        let line = line.context("Failed to read at least one line from input")?;
        let tokens: Vec<_> = line.split_whitespace().collect();
        match *tokens {
            [symbol, "is", roman] => {
                if !symbol_to_romans.contains_key(symbol) {
                    symbol_to_romans.insert(symbol.to_owned(), roman.parse()?);
                }
            }
            [ref symbols.., product, "is", credits, "Credits"] => {
                let credits = credits.parse::<f32>().with_context(|_| {
                    format!("Could not parse floating point number from '{}'", credits)
                })?;
                if !product_prices.contains_key(product) {
                    let product_price =
                        credits / symbols_to_decimal(&symbol_to_romans, symbols.iter())? as f32;
                    product_prices.insert(product.to_owned(), product_price);
                }
            }
            ["how", "much", "is", ref symbols.., "?"] => {
                let decimal_value = symbols_to_decimal(&symbol_to_romans, symbols.iter())?;
                writeln!(output, "{} is {}", symbols.join(" "), decimal_value)?;
            }
            ["how", "many", "Credits", "is", ref symbols.., product, "?"] => {
                let single_product_price = product_prices.get(product).ok_or_else(|| {
                    format_err!("Product named '{}' was not yet encountered", product)
                })?;
                let decimal_multiplier = symbols_to_decimal(&symbol_to_romans, symbols.iter())?;
                let product_price = decimal_multiplier as f32 * single_product_price;
                writeln!(
                    output,
                    "{} {} is {} Credits",
                    symbols.join(" "),
                    product,
                    product_price
                )?;
            }
            ["how", "much", _.., "?"] => {
                writeln!(output, "I have no idea what you are talking about")?;
            }
            _ => {
                return Err(format_err!("'{}' could not be parsed", line));
            }
        }
    }
    Ok(())
}
