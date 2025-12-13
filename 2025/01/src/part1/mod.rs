use crate::puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut position = 50isize;
    let mut count = 0usize;
    for turn in puzzle.turns {
        position = (position + turn.delta()).rem_euclid(100);
        if position == 0isize {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 3);
    }
}
