mod cell {
    use game_grid::ParseCellError;

    #[derive(Clone, Copy, Default, Debug, game_grid::GridCell)]
    pub enum Cell {
        #[default]
        #[cell('.')]
        Unoccupied,
        #[cell('@')]
        Occupied,
        #[cell('x')]
        Accessible,
    }
}

use cell::Cell;

#[derive(Debug, parse_display::FromStr)]
#[display("{grid}")]
pub struct Puzzle {
    pub grid: game_grid::Grid<Cell>,
}

impl Puzzle {
    fn is_occupied(&self, pos: position::Position) -> bool {
        self.grid.is_in_bounds(pos)
            && matches!(self.grid.cell_at(pos), Cell::Occupied | Cell::Accessible)
    }

    fn is_accessible(&self, pos: position::Position) -> bool {
        (-1i32..=1)
            .flat_map(|dx| (-1i32..=1).map(move |dy| (dx, dy)))
            .filter(|&(dx, dy)| {
                (dx, dy) != (0, 0)
                    && self.is_occupied(position::Position {
                        x: pos.x + dx,
                        y: pos.y + dy,
                    })
            })
            .take(4)
            .count()
            < 4
    }

    pub fn mark_accessible_cells(&mut self) {
        for i in 0..self.grid.len() {
            let pos = self.grid.position_for_index::<position::Position>(i);
            if self.is_occupied(pos) && self.is_accessible(pos) {
                self.grid.set_cell(pos, Cell::Accessible);
            }
        }
    }

    pub fn count_accessible_cells(&self) -> usize {
        self.grid
            .cells()
            .filter(|&cell| matches!(cell, Cell::Accessible))
            .count()
    }

    pub fn remove_accessible_cells(&mut self) {
        for cell in self.grid.mut_cells() {
            if let Cell::Accessible = cell {
                *cell = Cell::Unoccupied;
            }
        }
    }
}
