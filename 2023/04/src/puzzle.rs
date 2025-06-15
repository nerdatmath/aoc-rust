use parse_display::{Display, FromStr, ParseError};
use parse_display_with::formats::{delimiter, fmt_from_str};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, FromStr)]
pub struct Puzzle {
    #[display(with=delimiter("\n"))]
    pub cards: Vec<Card>,
}

#[derive(Debug, PartialEq, FromStr)]
#[display("Card {_n}: {winning} | {have}")]
#[from_str(regex = r"Card\s+(?<_n>[0-9]+): (?<winning>[ 0-9]+) \| (?<have>[ 0-9]+)")]
pub struct Card {
    pub _n: usize,
    #[display(with=fmt_from_str(from_str_delimited_by_ascii_whitespace))]
    pub winning: HashSet<Number>,
    #[display(with=fmt_from_str(from_str_delimited_by_ascii_whitespace))]
    pub have: Vec<Number>,
}

impl Card {
    pub fn wins(&self) -> usize {
        self.have
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

#[derive(Clone, Copy, Debug, Display, FromStr, PartialEq, Eq, Hash)]
pub struct Number(#[from_str(regex = r"\s*(?<>[0-9]+)")] u8);

fn from_str_delimited_by_ascii_whitespace<V, T>(s: &str) -> Result<V, ParseError>
where
    T: FromStr<Err = ParseError>,
    Result<V, ParseError>: FromIterator<Result<T, ParseError>>,
{
    s.split_ascii_whitespace().map(|s| s.parse()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_parse() {
        assert_eq!(" 3".parse::<Number>().unwrap(), Number(3));
    }

    #[test]
    fn test_number_display() {
        assert_eq!(Number(3).to_string(), " 3");
    }

    #[test]
    fn test_card_parse() {
        assert_eq!(
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"
                .parse::<Card>()
                .unwrap(),
            Card {
                _n: 3,
                winning: [1, 21, 53, 59, 44].into_iter().map(Number).collect(),
                have: [69, 82, 63, 72, 16, 21, 14, 1]
                    .into_iter()
                    .map(Number)
                    .collect(),
            }
        )
    }
}
