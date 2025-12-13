use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug)]
pub struct Puzzle {
    pub turns: Vec<Turn>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    L,
    R,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Direction::L,
            "R" => Direction::R,
            _ => return Err(ParseError),
        })
    }
}

#[derive(Debug)]
pub struct Turn {
    pub direction: Direction,
    pub count: usize,
}

impl Turn {
    pub fn delta(&self) -> isize {
        let count: isize = self.count.try_into().unwrap();
        (match self.direction {
            Direction::L => -1isize,
            Direction::R => 1isize,
        }) * count
    }
}

impl FromStr for Turn {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Turn {
            direction: s.get(0..1).ok_or(ParseError)?.parse()?,
            count: s
                .get(1..)
                .ok_or(ParseError)?
                .parse()
                .map_err(|_| ParseError)?,
        })
    }
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            turns: s.lines().map(|s| s.parse()).collect::<Result<_, _>>()?,
        })
    }
}
