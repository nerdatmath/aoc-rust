use direction::Direction;
use game_grid::GridPosition;
use parse_display::Display;

#[derive(Clone, Copy, Debug, Display, GridPosition, PartialEq, Eq, Hash)]
#[display("({x},{y})")]
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
        let (dx, dy): (i32, i32) = rhs.into();
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        let (dx, dy): (i32, i32) = rhs.into();
        self.x += dx;
        self.y += dy;
    }
}
