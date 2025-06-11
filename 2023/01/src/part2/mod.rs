use crate::puzzle::Puzzle;
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug)]
struct NoDigitError<'a>(#[allow(dead_code)] &'a str);

const DIGITS: &str = r"([0-9]|one|two|three|four|five|six|seven|eight|nine)";

fn from_digit(s: &str) -> Result<u64, NoDigitError> {
    Ok(match s {
        "0" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => return Err(NoDigitError(s)),
    })
}

fn first_digit(s: &str) -> Result<u64, NoDigitError> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(&[r"^.*?", DIGITS, r".*$"].join("")).unwrap());
    let captures = RE.captures(s).ok_or(NoDigitError(s))?;
    let m = captures.get(1).unwrap();
    from_digit(m.as_str())
}

fn last_digit(s: &str) -> Result<u64, NoDigitError> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(&[r"^.*", DIGITS, r".*?$"].join("")).unwrap());
    let captures = RE.captures(s).ok_or(NoDigitError(s))?;
    let m = captures.get(1).unwrap();
    from_digit(m.as_str())
}

fn calibration_value(s: &str) -> Result<u64, NoDigitError> {
    Ok(first_digit(s)? * 10 + last_digit(s)?)
}

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .lines
        .into_iter()
        .map(|s| calibration_value(&s).expect("bad input"))
        .sum()
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE2;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE2), 281);
    }
}
