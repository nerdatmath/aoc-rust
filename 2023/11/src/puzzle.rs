use crate::distance_sum::distance_sum;
use game_grid::{Grid, GridCell, ParseCellError};
use parse_display::FromStr;
use position::Position;

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Cell {
    #[cell('#')]
    Galaxy,
    #[cell('.')]
    #[default]
    Empty,
}

#[derive(Debug, FromStr)]
#[display("{galaxies}")]
pub struct Puzzle {
    pub galaxies: Grid<Cell>,
}

impl Puzzle {
    fn galaxy_counts(&self) -> (Vec<u64>, Vec<u64>) {
        let galaxies = &self.galaxies;
        let mut rows = vec![0u64; galaxies.height()];
        let mut cols = vec![0u64; galaxies.width()];
        for (pos, cell) in galaxies.iter::<Position>() {
            if cell != Cell::Galaxy {
                continue;
            }
            let col: usize = pos.x.try_into().unwrap();
            let row: usize = pos.y.try_into().unwrap();
            rows[row] += 1;
            cols[col] += 1;
        }
        (rows, cols)
    }

    pub fn distance_sum_2d(&self, factor: u64) -> u64 {
        let (rows, cols) = self.galaxy_counts();
        distance_sum(rows, factor) + distance_sum(cols, factor)
    }
}
