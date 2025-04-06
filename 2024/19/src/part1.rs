use crate::paths;
use crate::puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .patterns
        .into_iter()
        .filter(|pattern| paths::has_path(&pattern, &puzzle.towels))
        .count()
}

#[cfg(test)]
mod test {
    use super::run;

    const EXAMPLE1: &'static str = include_str!("../data/example1");

    #[test]
    fn test_example1() {
        assert_eq!(run(EXAMPLE1), 6);
    }
}
