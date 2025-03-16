use derive_more::{FromStr, TryFrom};
use parse_display::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, TryFrom, FromStr, Hash)]
#[try_from(repr)]
#[repr(u8)]
pub enum Direction {
    #[display("^")]
    N = b'^',
    #[display("v")]
    S = b'v',
    #[display(">")]
    E = b'>',
    #[display("<")]
    W = b'<',
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
        }
    }
}

#[derive(Debug)]
pub struct InvalidDirectionError;

impl TryFrom<(i32, i32)> for Direction {
    type Error = InvalidDirectionError;

    fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
        Ok(match value {
            (0, -1) => Direction::N,
            (0, 1) => Direction::S,
            (1, 0) => Direction::E,
            (-1, 0) => Direction::W,
            _ => Err(InvalidDirectionError)?,
        })
    }
}

#[derive(Debug)]
pub struct ParseDirError;

impl TryFrom<char> for Direction {
    type Error = ParseDirError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        Ok(Self::try_from(u8::try_from(ch).map_err(|_| ParseDirError)?)
            .map_err(|_| ParseDirError)?)
    }
}

impl Direction {
    pub fn is_vertical(&self) -> bool {
        let (dx, _dy): (i32, i32) = (*self).into();
        dx == 0
    }
    pub fn rotr(&self) -> Self {
        let (dx, dy): (i32, i32) = (*self).into();
        Self::try_from((-dy, dx)).unwrap()
    }
    pub fn rotl(&self) -> Self {
        let (dx, dy): (i32, i32) = (*self).into();
        Self::try_from((dy, -dx)).unwrap()
    }
    pub fn reverse(&self) -> Self {
        let (dx, dy): (i32, i32) = (*self).into();
        Self::try_from((-dx, -dy)).unwrap()
    }
}
