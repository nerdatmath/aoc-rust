use crate::find_path::find_path;
use crate::puzzle::{Position, Puzzle};
use pathfinding::grid::Grid;

pub fn run(input: &str, size: usize) -> Position {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut grid: Grid = Grid::new(size + 1, size + 1);
    grid.fill();
    let mut path = find_path(&grid, (0, 0), (size, size)).expect("no path to start with.");
    for &Position { x, y } in &puzzle.positions {
        grid.remove_vertex((x, y));
        if path.contains(&(x, y)) {
            match find_path(&grid, (0, 0), (size, size)) {
                Some(new_path) => path = new_path,
                None => return Position { x, y },
            }
        }
    }
    panic!("nothing blocked our path")
}

#[cfg(test)]
mod test {
    use super::{Position, run};

    const EXAMPLE1: &'static str = include_str!("../data/example1");

    #[test]
    fn test_example1() {
        assert_eq!(run(EXAMPLE1, 6), Position::new(6, 1));
    }
}
