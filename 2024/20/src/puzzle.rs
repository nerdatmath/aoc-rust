use pathfinding::{grid::Grid, matrix::Matrix};
use std::{
    fmt::{Display, Formatter, Write as _},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    #[inline]
    pub fn vertex(&self) -> (usize, usize) {
        (self.col, self.row)
    }
    pub fn from_vertex((x, y): (usize, usize)) -> Self {
        Coord { row: y, col: x }
    }
}

/// From (row, col) to Coord.
impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0,
            col: value.1,
        }
    }
}

/// From Coord to (row, col).
impl From<Coord> for (usize, usize) {
    fn from(value: Coord) -> Self {
        (value.row, value.col)
    }
}

pub struct Puzzle {
    pub grid: Grid,
    pub start: Coord,
    pub end: Coord,
}

#[derive(Debug)]
pub enum ParseError {
    MatrixFormatError,
    MissingStart,
    MissingEnd,
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mat = Matrix::from_rows(s.lines().map(|l| l.chars()))
            .map_err(|_| ParseError::MatrixFormatError)?;
        let source = mat
            .items()
            .find_map(|(pos, &ch)| (ch == 'S').then_some(pos))
            .ok_or(ParseError::MissingStart)?;
        let target = mat
            .items()
            .find_map(|(pos, &ch)| (ch == 'E').then_some(pos))
            .ok_or(ParseError::MissingEnd)?;
        let grid: Grid = mat.map(|ch| ch != '#').into();
        Ok(Self {
            grid,
            start: Coord {
                row: source.0,
                col: source.1,
            },
            end: Coord {
                row: target.0,
                col: target.1,
            },
        })
    }
}

pub trait Map {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn char_at(&self, coord: Coord) -> char;
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in 0..self.height() {
            for col in 0..self.width() {
                f.write_char(self.char_at(Coord { row, col }))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Map for Puzzle {
    fn height(&self) -> usize {
        self.grid.height
    }

    fn width(&self) -> usize {
        self.grid.width
    }

    fn char_at(&self, coord: Coord) -> char {
        match coord {
            coord if coord == self.start => 'S',
            coord if coord == self.end => 'E',
            coord if self.grid.has_vertex(coord.vertex()) => '.',
            _ => '#',
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Map::fmt(self, f)
    }
}
