use direction::Direction;
use game_grid::GridPosition;

#[derive(Clone, Copy, Debug, GridPosition, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add<Direction> for &Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        let (dx, dy): (i32, i32) = rhs.into();
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}
