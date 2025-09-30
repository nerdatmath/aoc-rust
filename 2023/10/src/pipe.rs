use direction::Direction;
use game_grid::GridCell;
use game_grid::ParseCellError;

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Pipe {
    #[cell('.')]
    #[default]
    Ground,
    #[cell('|')]
    NorthSouth,
    #[cell('-')]
    EastWest,
    #[cell('L')]
    NorthEast,
    #[cell('J')]
    NorthWest,
    #[cell('7')]
    SouthWest,
    #[cell('F')]
    SouthEast,
    #[cell('S')]
    Start,
}

impl Pipe {
    pub fn next_direction(&self, direction: Direction) -> Option<Direction> {
        use Direction::*;
        use Pipe::*;
        Some(match (*self, direction) {
            (NorthSouth, N) => N,
            (NorthSouth, S) => S,
            (EastWest, E) => E,
            (EastWest, W) => W,
            (NorthEast, S) => E,
            (NorthEast, W) => N,
            (NorthWest, S) => W,
            (NorthWest, E) => N,
            (SouthWest, N) => W,
            (SouthWest, E) => S,
            (SouthEast, N) => E,
            (SouthEast, W) => S,
            _ => return None,
        })
    }

    pub fn has_direction(&self, direction: Direction) -> bool {
        self.next_direction(direction).is_some()
    }
}
