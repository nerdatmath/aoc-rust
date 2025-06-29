use crate::puzzle::{Part, Puzzle, Symbol};
use std::collections::HashMap;

fn adjacent_to_symbol(part: &Part, symbols: &HashMap<(usize, usize), Symbol>) -> bool {
    part.adjacent_points()
        .into_iter()
        .any(|pos| symbols.contains_key(&pos))
}

pub fn run(input: &str) -> u32 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .parts
        .into_iter()
        .filter(|part| adjacent_to_symbol(part, &puzzle.symbols))
        .map(|part| part.number)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 4361);
    }
}
