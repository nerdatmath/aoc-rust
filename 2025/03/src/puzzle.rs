use std::str::FromStr;

#[derive(Debug)]
pub struct Puzzle {
    pub banks: Box<[Bank]>,
}

#[derive(Debug)]
pub struct ParseError;

impl From<std::num::ParseIntError> for ParseError {
    fn from(_value: std::num::ParseIntError) -> Self {
        Self
    }
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            banks: s.lines().map(|s| s.parse()).collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
pub struct Bank {
    pub batteries: Box<[Battery]>,
}

impl FromStr for Bank {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bank {
            batteries: s
                .chars()
                .map(|c| c.to_string().parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
pub struct Battery {
    pub joltage: u64,
}

impl FromStr for Battery {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Battery {
            joltage: s.parse()?,
        })
    }
}
