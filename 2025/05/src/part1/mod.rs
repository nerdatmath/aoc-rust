use crate::intervals::Intervals;
use crate::puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let intervals = Intervals::new(&puzzle.fresh_ranges);
    puzzle
        .available
        .iter()
        .cloned()
        .filter(|item| intervals.contains(item))
        .count()
}

#[test]
fn test1() {
    assert_eq!(run(crate::data::EXAMPLE1), 3);
}
