use crate::puzzle::Puzzle;

pub fn run(input: &str) -> i64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle.histories.iter().map(|h| h.next_value()).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 114);
    }
}
