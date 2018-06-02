use std::str::FromStr;
use std::io::BufReader;

pub fn answer(input: impl std::io::Read) -> Result<(), String> {
    use std::io::BufRead;
    let lines = BufReader::new(input).lines();
    let mut symbols = std::collections::HashMap::<String, Roman>::new();
    for (index, line) in lines.enumerate() {
        let line = line.map_err(|_err| format!("Error with line: {}", index))?;
        let mut words = line.split_whitespace();
        match (words.next(), words.next(), words.next()) {
            (Some(symbol), Some("is"), Some(roman)) => {
                symbols.insert(symbol.to_owned(), roman.parse()?);
            }
            _ => unimplemented!(),
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
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
    type Err = String;

    fn from_str(ch: &str) -> Result<Roman, String> {
        use self::Roman::*;
        Ok(match ch {
            "I" => I,
            "V" => V,
            "X" => X,
            "L" => L,
            "C" => C,
            "D" => D,
            "M" => M,
            _ => return Err(format!("invalid input {}", ch)),
        })
    }
}

impl From<Roman> for u16 {
    fn from(r: Roman) -> u16 {
        use self::Roman::*;
        match r {
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

#[test]
fn should_map_string_to_roman() {
    assert_eq!(("I").parse(), Ok(Roman::I));
    assert_eq!(("N").parse::<Roman>(), Err("invalid input N".to_string()));
    assert_eq!(u16::from(Roman::I), 1u16);
}
