use parse_display::{Display, FromStr};
use parse_display_with::formats::delimiter;

#[derive(Debug, Display, FromStr)]
pub struct Puzzle {
    #[display(with=delimiter("\n"))]
    pub positions: Vec<Position>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, FromStr)]
#[display("{x},{y}")]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<Position> for (usize, usize) {
    fn from(value: Position) -> Self {
        (value.x, value.y)
    }
}
