use std::str::FromStr;

use crate::direction::{self, Direction};

#[derive(Debug)]
pub struct ParseError;

impl From<direction::ParseError> for ParseError {
    fn from(_value: direction::ParseError) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct Instructions(pub Box<[Direction]>);

impl FromStr for Instructions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(Direction::try_from)
                .collect::<Result<_, _>>()?,
        ))
    }
}
