use itertools::Itertools as _;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Write};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Computer(pub [u8; 2]);

#[derive(Debug)]
pub struct ParseComputerError;

impl FromStr for Computer {
    type Err = ParseComputerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 || !s.is_ascii() {
            return Err(ParseComputerError);
        }
        Ok(Self(s.bytes().collect_array().unwrap()))
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            f.write_char(b.into())?
        }
        Ok(())
    }
}

impl Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Connection {
    pub a: Computer,
    pub b: Computer,
}

#[derive(Debug)]
pub struct ParseConnectionError;

impl From<(Computer, Computer)> for Connection {
    fn from((a, b): (Computer, Computer)) -> Self {
        if a > b {
            Connection { a: b, b: a }
        } else {
            Connection { a, b }
        }
    }
}

impl FromStr for Connection {
    type Err = ParseConnectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').ok_or(ParseConnectionError)?;
        let a = a.parse().map_err(|_| ParseConnectionError)?;
        let b = b.parse().map_err(|_| ParseConnectionError)?;
        Ok((a, b).into())
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.a, self.b)
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub connections: HashSet<Connection>,
}

#[derive(Debug)]
pub struct ParsePuzzleError;

impl FromStr for Puzzle {
    type Err = ParsePuzzleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            connections: s
                .lines()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()
                .map_err(|_| ParsePuzzleError)?,
        })
    }
}

impl Puzzle {
    pub fn connected(&self, a: Computer, b: Computer) -> bool {
        self.connections.contains(&(a, b).into())
    }
}
