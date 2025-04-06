use crate::find_path::find_path;
use crate::puzzle::{Position, Puzzle};
use pathfinding::grid::Grid;

pub fn run(input: &str, size: usize, count: usize) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut grid: Grid = Grid::new(size + 1, size + 1);
    grid.fill();
    for &Position { x, y } in &puzzle.positions[..count] {
        grid.remove_vertex((x, y));
    }
    if let Some(path) = find_path(&grid, (0, 0), (size, size)) {
        path.len() - 1
    } else {
        panic!("no path")
    }
}

#[cfg(test)]
mod test {
    use super::run;

    const EXAMPLE1: &'static str = include_str!("../data/example1");

    #[test]
    fn test_example1() {
        assert_eq!(run(EXAMPLE1, 6, 12), 22);
    }
}
