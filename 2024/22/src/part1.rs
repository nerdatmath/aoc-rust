use crate::puzzle::Puzzle;
use crate::secret::next;

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .numbers
        .into_iter()
        .map(|mut n| {
            for _ in 0..2000 {
                n = next(n);
            }
            n as u64
        })
        .sum()
}

#[cfg(test)]
pub mod test {
    use super::run;

    const EXAMPLE1: &'static str = include_str!("../data/example1");

    #[test]
    fn test_example1() {
        assert_eq!(run(EXAMPLE1), 37327623);
    }
}
