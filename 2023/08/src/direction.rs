#[derive(Debug)]
pub struct ParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Left,
    Right,
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
