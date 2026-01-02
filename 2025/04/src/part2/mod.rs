use crate::puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let mut puzzle: Puzzle = input.parse().expect("parse failed");
    let mut removed = 0usize;
    loop {
        puzzle.mark_accessible_cells();
        match puzzle.count_accessible_cells() {
            0 => break,
            count => {
                puzzle.remove_accessible_cells();
                removed += count;
            }
        }
    }
    removed
}

#[test]
fn test_run() {
    assert_eq!(run(crate::data::EXAMPLE1), 43);
}
