use enum_iterator::Sequence;
use parse_display::Display;

#[derive(Clone, Copy, Display, PartialEq, Eq, Hash, Debug, Sequence)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    pub fn apply(self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Direction::U => (row.checked_sub(1)?, col),
            Direction::D => (row.checked_add(1)?, col),
            Direction::L => (row, col.checked_sub(1)?),
            Direction::R => (row, col.checked_add(1)?),
        })
    }
}
