use crate::puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let mut puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle.mark_accessible_cells();
    puzzle.count_accessible_cells()
}

#[test]
fn test1() {
    assert_eq!(run(crate::data::EXAMPLE1), 13);
}
