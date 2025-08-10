use crate::puzzle::Puzzle;

#[derive(Debug)]
struct NoDigitError<'a>(#[allow(dead_code)]&'a str);

fn first_digit(s: &'_ str) -> Result<u64, NoDigitError<'_>> {
    for ch in s.chars() {
        match ch {
            '0'..='9' => return Ok(u64::from(ch) - u64::from('0')),
            _ => continue,
        }
    }
    Err(NoDigitError(s))
}

fn last_digit(s: &'_ str) -> Result<u64, NoDigitError<'_>> {
    for ch in s.chars().rev() {
        match ch {
            '0'..='9' => return Ok(u64::from(ch) - u64::from('0')),
            _ => continue,
        }
    }
    Err(NoDigitError(s))
}

fn calibration_value(s: &'_ str) -> Result<u64, NoDigitError<'_>> {
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
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 142);
    }
}
