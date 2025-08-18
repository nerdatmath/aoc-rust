use std::ops::{Add, Mul, Sub};

use direction::Direction;
use game_grid::GridPosition;

#[derive(Debug, Clone, Copy, GridPosition)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl<T: Into<(i32, i32)>> Add<T> for Position {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let (dx, dy) = rhs.into();
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl<T: Into<(i32, i32)>> Sub<T> for Position {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let (dx, dy) = rhs.into();
        Position {
            x: self.x - dx,
            y: self.y - dy,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Position {
    pub fn next(&self, direction: Direction) -> Self {
        *self + direction
    }

    pub fn step(&mut self, direction: Direction) {
        *self = self.next(direction);
    }
}
