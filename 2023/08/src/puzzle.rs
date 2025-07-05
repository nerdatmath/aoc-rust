use lazy_regex::regex_if;
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct Puzzle {
    pub instructions: Box<[Direction]>,
    pub nodes: HashMap<Node, (Node, Node)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node([u8; 3]);

impl Node {
    pub fn is_target(&self) -> bool {
        self.0[0] == b'Z'
    }
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, nodes) = s.split_once("\n\n").ok_or(ParseError)?;
        Ok(Puzzle {
            instructions: instructions
                .chars()
                .map(|ch| ch.try_into())
                .collect::<Result<_, _>>()?,
            nodes: nodes
                .lines()
                .map(|s| {
                    regex_if!(
                        r#"(?<source>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)"#,
                        s,
                        (source.parse()?, (left.parse()?, right.parse()?))
                    )
                    .ok_or(ParseError)
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<char> for Direction {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return Err(ParseError),
        })
    }
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseError);
        }
        Ok(Node(s.as_bytes().try_into().map_err(|_| ParseError)?))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}
