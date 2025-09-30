use std::str::FromStr;

use crate::history::History;

#[derive(Debug)]
pub struct ParseError;

impl From<<History as FromStr>::Err> for ParseError {
    fn from(_value: <History as FromStr>::Err) -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub histories: Box<[History]>,
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            histories: s.lines().map(|s| s.parse()).collect::<Result<_, _>>()?,
        })
    }
}
