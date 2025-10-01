use crate::*;
use anyhow::Result;
use std::str::FromStr;
use stone::Stone;

#[derive(Debug)]
pub struct Puzzle(pub Vec<Stone>);

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Puzzle(
            s.split(' ')
                .map(|s| Ok(Stone(s.parse()?)))
                .collect::<Result<_>>()?,
        ))
    }
}
