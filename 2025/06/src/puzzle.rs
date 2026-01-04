use std::str::FromStr;

#[derive(Debug)]
pub struct Puzzle {
    pub problems: Box<[Problem]>,
}

#[derive(Debug)]
pub struct Problem {
    pub inputs: Box<[u64]>,
    pub operation: Operation,
}

impl Problem {
    pub fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.inputs.iter().sum(),
            Operation::Multiply => self.inputs.iter().product(),
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct ParseError;

impl From<std::num::ParseIntError> for ParseError {
    fn from(_value: std::num::ParseIntError) -> Self {
        ParseError
    }
}

impl FromStr for Problem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let operation: Operation = lines.next_back().ok_or(ParseError)?.parse()?;
        let inputs: Box<[u64]> = lines.map(|s| s.parse()).collect::<Result<_, _>>()?;
        Ok(Problem { inputs, operation })
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => return Err(ParseError),
        })
    }
}
