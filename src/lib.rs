#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use failure::{Error, ResultExt};
use std::{io::{BufRead, BufReader, Read, Write}, str::FromStr};

lazy_static! {
    static ref RE_SET_ROMAN: regex::Regex =
        regex::Regex::new(r"(?i)^\s*(?P<symbol>\w+)\s+is\s+(?P<roman>\w)\s*$").expect("valid regex");
    static ref RE_ASSIGN_CREDITS: regex::Regex = regex::Regex::new(
        r"(?i)^\s*(?P<symbols>[\w\s]+)\s+(?P<product>\w+)\s+is\s+(?P<credits>\d+)\s+credits$"
    ).expect("valid regex");
    static ref RE_QUERY_ROMAN: regex::Regex = regex::Regex::new(
        r"(?i)^\s*how\s*much\s+is\s+(?P<symbols>[\w\s]+).*$"
    ).expect("valid regex");
    static ref RE_QUERY_PRODUCT: regex::Regex = regex::Regex::new(
        r"(?i)^\s*how\s*many\s+credits\s+is\s+(?P<symbols>[\w\s]+)\s+(?P<product>\w+).*$"
    ).expect("valid regex");
    static ref RE_QUERY_OTHER: regex::Regex =
        regex::Regex::new(r"(?i)^\s*how\s*much\s+.*$").expect("valid regex");
}

fn roman_to_decimal(romans: &[Roman]) -> Result<u32, Error> {
    Ok(match romans.len() {
        0 => bail!("Cannot compute decimal for missing roman value"),
        1 => romans[0].into(),
        _ => {
            let mut decimal = 0;
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
                decimal += c as i64 * multiplier;
            }
            if decimal < 0 {
                bail!(
                    "Converted '{:?}' into negative decimal value {}",
                    romans,
                    decimal
                );
            }
            decimal as u32
        }
    })
}

#[derive(Debug, Default)]
struct ConversionTable {
    symbol_to_romans: Vec<(String, Roman)>,
    product_prices: Vec<(String, f32)>,
}

impl ConversionTable {
    fn symbols_to_romans(&self, values_space_separated: &str) -> Result<Vec<Roman>, Error> {
        values_space_separated
            .split_whitespace()
            .map(|s| {
                find_by_name(&self.symbol_to_romans, s)
                    .ok_or_else(|| format_err!("No roman value was associated with symbol '{}'", s))
            })
            .collect()
    }

    fn symbols_to_decimal(&self, symbol_space_separated: &str) -> Result<u32, Error> {
        roman_to_decimal(&self.symbols_to_romans(symbol_space_separated)?)
    }

    fn update(
        &mut self,
        tokens: impl Iterator<Item = Result<Token, Error>>,
    ) -> Result<Vec<Query>, Error> {
        use self::Token::*;
        let mut queries = Vec::new();
        for token in tokens {
            match token {
                Ok(token) => match token {
                    Other(query) => queries.push(query),
                    RomanNumeralMapping { symbol, roman } => {
                        self.symbol_to_romans.push((symbol, roman));
                    }
                    PriceAssignment {
                        credits,
                        product,
                        symbols_space_separated,
                    } => {
                        let product_price =
                            credits / self.symbols_to_decimal(&symbols_space_separated)? as f32;
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
    Other,
    Roman {
        symbols_space_separated: String,
    },
    Product {
        symbols_space_separated: String,
        product: String,
    },
}

fn find_by_name<T: Clone>(from: &[(String, T)], what: &str) -> Option<T> {
    from.iter()
        .find(|(n, _v)| n == what)
        .map(|(_n, v)| v.clone())
}

impl Query {
    fn answer(&self, table: &ConversionTable) -> Result<String, Error> {
        use self::Query::*;
        Ok(match self {
            Other => String::from("I have no idea what you are talking about"),
            Product {
                symbols_space_separated,
                product,
            } => {
                let single_product_price = find_by_name(&table.product_prices, &product)
                    .ok_or_else(|| {
                        format_err!("Product named '{}' was not yet encountered", product)
                    })?;
                let decimal_multiplier = table.symbols_to_decimal(&symbols_space_separated)?;
                let product_price = decimal_multiplier as f32 * single_product_price;
                format!(
                    "{} {} is {} Credits",
                    symbols_space_separated, product, product_price
                )
            }
            Roman {
                symbols_space_separated,
            } => {
                let decimal_value = table.symbols_to_decimal(&symbols_space_separated)?;
                format!("{} is {}", symbols_space_separated, decimal_value)
            }
        })
    }
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
        symbol: String,
        roman: Roman,
    },
    PriceAssignment {
        credits: f32,
        product: String,
        symbols_space_separated: String,
    },
    Other(Query),
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Token, Error> {
        use self::Token::*;
        Ok(if let Some(captures) = RE_SET_ROMAN.captures(s) {
            RomanNumeralMapping {
                symbol: captures["symbol"].to_owned(),
                roman: captures["roman"].parse()?,
            }
        } else if let Some(captures) = RE_ASSIGN_CREDITS.captures(s) {
            PriceAssignment {
                credits: captures["credits"].parse::<f32>().with_context(|_| {
                    format!(
                        "Could not parse floating point number from '{}'",
                        &captures["credits"]
                    )
                })?,
                product: captures["product"].to_owned(),
                symbols_space_separated: captures["symbols"].to_owned(),
            }
        } else if let Some(captures) = RE_QUERY_ROMAN.captures(s) {
            Other(Query::Roman {
                symbols_space_separated: captures["symbols"].trim_right().to_owned(),
            })
        } else if let Some(captures) = RE_QUERY_PRODUCT.captures(s) {
            Other(Query::Product {
                symbols_space_separated: captures["symbols"].to_owned(),
                product: captures["product"].to_owned(),
            })
        } else if RE_QUERY_OTHER.is_match(s) {
            Other(Query::Other)
        } else {
            return Err(format_err!("'{}' could not be parsed", s));
        })
    }
}

fn parse(input: impl Read) -> impl Iterator<Item = Result<Token, Error>> {
    let input = BufReader::new(input);
    input.lines().map(|r| {
        r.context("Failed to read at least one line from input")
            .map_err(Into::into)
            .and_then(|line| Token::from_str(&line))
    })
}

pub fn answers(input: impl Read, mut output: impl Write) -> Result<(), Error> {
    let mut table = ConversionTable::default();
    let queries = table.update(parse(input))?;
    if queries.is_empty() {
        writeln!(output, "{:#?}", table)?;
    } else {
        for query in queries {
            writeln!(output, "{}", query.answer(&table)?)?;
        }
    }
    Ok(())
}
