use game_grid::{Grid, GridCell, ParseCellError, ParseGridError};
use position::Position;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, GridCell)]
pub enum Cell {
    #[cell('O')]
    RoundRock,
    #[cell('#')]
    CubeRock,
    #[default]
    #[cell('.')]
    Empty,
}

#[derive(Debug)]
pub struct Puzzle {
    grid: Grid<Cell>,
}

impl Puzzle {
    pub fn load_north(&self) -> usize {
        let mut total_load = 0usize;
        for col in 0..self.grid.width() {
            let mut load = self.grid.height();
            for row in 0..self.grid.height() {
                let position = Position {
                    y: i32::try_from(row).unwrap(),
                    x: i32::try_from(col).unwrap(),
                };
                match self.grid.cell_at(position) {
                    Cell::RoundRock => {
                        total_load += load;
                        load -= 1;
                    }
                    Cell::CubeRock => {
                        load = self.grid.height() - row - 1;
                    }
                    Cell::Empty => (),
                }
            }
        }
        total_load
    }
}

#[derive(Debug)]
pub struct ParseError;

impl From<ParseCellError> for ParseError {
    fn from(_value: ParseCellError) -> Self {
        ParseError
    }
}

impl From<ParseGridError<ParseCellError>> for ParseError {
    fn from(_value: ParseGridError<ParseCellError>) -> Self {
        ParseError
    }
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle { grid: s.parse()? })
    }
}
