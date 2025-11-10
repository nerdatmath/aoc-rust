use bitvec::boxed::BitBox;
use bitvec::prelude::*;
use game_grid::{Grid, GridCell, ParseCellError, ParseGridError};
use position::Position;
use std::{cmp::min, str::FromStr};

#[derive(Debug)]
pub struct Pattern(pub Vec<BitBox>);

impl Pattern {
    pub fn find_mirror(&self, smudges: usize) -> Option<usize> {
        for m in 1..self.0.len() {
            let n: usize = (0..min(m, self.0.len() - m))
                .into_iter()
                .map(|i| {
                    let diff = self.0[m - i - 1].clone() ^ self.0[m + i].clone();
                    diff.count_ones()
                })
                .sum();
            if n == smudges {
                return Some(m);
            }
        }
        None
    }
}

#[test]
fn test_find_mirror() {
    let pattern = Pattern(vec![
        bitbox![1, 0, 1, 1, 0, 0, 1],
        bitbox![0, 0, 1, 1, 0, 0, 0],
        bitbox![1, 1, 0, 0, 1, 1, 1],
        bitbox![1, 0, 0, 0, 0, 1, 0],
        bitbox![0, 1, 0, 0, 1, 0, 1],
        bitbox![0, 1, 0, 0, 1, 0, 1],
        bitbox![1, 0, 0, 0, 0, 1, 0],
        bitbox![1, 1, 0, 0, 1, 1, 1],
        bitbox![0, 0, 1, 1, 0, 0, 0],
    ]);
    assert_eq!(pattern.find_mirror(0), Some(5));
}

#[derive(Debug)]
pub struct Field {
    pub horizontal: Pattern,
    pub vertical: Pattern,
}

impl Field {
    pub fn summary(&self, smudges: usize) -> usize {
        if let Some(m) = self.vertical.find_mirror(smudges) {
            m
        } else if let Some(m) = self.horizontal.find_mirror(smudges) {
            100 * m
        } else {
            panic!("no mirror found");
        }
    }
}

impl FromStr for Field {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, GridCell)]
        enum Cell {
            #[default]
            #[cell('.')]
            Ash,
            #[cell('#')]
            Rocks,
        }

        let grid: Grid<Cell> = s.parse()?;
        let mut horizontal: Pattern = Pattern(vec![bitbox![0;grid.width()]; grid.height()]);
        let mut vertical: Pattern = Pattern(vec![bitbox![0;grid.height()]; grid.width()]);
        for (pos, cell) in grid.iter::<Position>() {
            let row: usize = pos.y.try_into().unwrap();
            let col: usize = pos.x.try_into().unwrap();
            let value = cell == Cell::Rocks;
            horizontal.0[row].set(col, value);
            vertical.0[col].set(row, value);
        }
        Ok(Field {
            horizontal,
            vertical,
        })
    }
}

impl Puzzle {
    pub fn solve(&self, smudges: usize) -> usize {
        self.0.iter().map(|field| field.summary(smudges)).sum()
    }
}

#[derive(Debug)]
pub struct Puzzle(pub Vec<Field>);

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle(
            s.split("\n\n")
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Debug)]
pub struct ParseError;

impl From<ParseCellError> for ParseError {
    fn from(_value: ParseCellError) -> Self {
        ParseError
    }
}

impl<T> From<ParseGridError<T>> for ParseError {
    fn from(_value: ParseGridError<T>) -> Self {
        ParseError
    }
}
