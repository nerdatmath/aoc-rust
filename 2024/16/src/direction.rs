use parse_display::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
pub enum Direction {
    N = 0,
    S = 180,
    E = 90,
    W = 270,
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

impl Direction {
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
