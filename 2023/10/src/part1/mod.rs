use crate::*;
use puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle.iter().count() / 2
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data;

    #[test]
    fn test1() {
        assert_eq!(run(data::EXAMPLE1), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(run(data::EXAMPLE2), 8);
    }
}
