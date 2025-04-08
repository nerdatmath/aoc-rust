use crate::cheats::summarize_cheats;
use crate::puzzle::Puzzle;

const MAX_JUMP_DISTANCE: usize = 20;
const MIN_SAVINGS: usize = 100;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    summarize_cheats(&puzzle, MAX_JUMP_DISTANCE, MIN_SAVINGS)
        .values()
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    const EXAMPLE1: &str = include_str!("../data/example1");

    #[test]
    fn test_1() {
        let puzzle: Puzzle = EXAMPLE1.parse().expect("parse failed");
        let want = HashMap::from([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ]);
        assert_eq!(summarize_cheats(&puzzle, MAX_JUMP_DISTANCE, 50), want);
    }
}
