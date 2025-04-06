use crate::puzzle::Puzzle;
use regex::Regex;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let re = Regex::new(&format!("^({})*$", puzzle.towels.join("|"))).unwrap();
    puzzle
        .patterns
        .into_iter()
        .filter(|pattern| re.is_match(pattern.as_str()))
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
