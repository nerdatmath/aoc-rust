use std::str::FromStr;

#[derive(Debug)]
pub struct Puzzle {
    pub lines: Vec<String>,
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            lines: s.lines().map(ToOwned::to_owned).collect(),
        })
    }
}
