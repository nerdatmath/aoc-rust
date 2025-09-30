use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct History(pub Box<[i64]>);

impl FromStr for History {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_ascii_whitespace()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl History {
    pub fn next_value(&self) -> i64 {
        if self.0.len() <= 1 {
            return 0;
        }
        let diffs = History(self.0.windows(2).map(|s| s[1] - s[0]).collect());
        self.0[self.0.len() - 1] + diffs.next_value()
    }

    pub fn prev_value(&self) -> i64 {
        if self.0.len() <= 1 {
            return 0;
        }
        let diffs = History(self.0.windows(2).map(|s| s[1] - s[0]).collect());
        self.0[0] - diffs.prev_value()
    }
}

#[test]
fn next_value() {
    let h = History(Box::new([0, 3, 6, 9, 12, 15]));
    assert_eq!(h.next_value(), 18);
}

#[test]
fn prev_value() {
    let h = History(Box::new([10, 13, 16, 21, 30, 45]));
    assert_eq!(h.prev_value(), 5);
}
