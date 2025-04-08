use crate::cheats::summarize_cheats;
use crate::puzzle::Puzzle;

const MAX_JUMP_DISTANCE: usize = 2;
const MIN_SAVINGS: usize = 100;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    summarize_cheats(&puzzle, MAX_JUMP_DISTANCE, MIN_SAVINGS)
        .values()
        .sum()
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::collections::HashMap;

    const EXAMPLE1: &'static str = include_str!("../data/example1");

    #[test]
    fn test_print_example1() {
        let puzzle: Puzzle = EXAMPLE1.parse().expect("parse failed");
        println!("{}", puzzle);
    }

    #[test]
    fn test_1() {
        let puzzle = EXAMPLE1.parse().expect("parse failed");
        let want = HashMap::from([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]);
        assert_eq!(summarize_cheats(&puzzle, MAX_JUMP_DISTANCE, 1), want);
    }
}
