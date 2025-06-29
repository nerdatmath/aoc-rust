use crate::puzzle::Puzzle;

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let (intervals, _) = puzzle.seeds.as_chunks();
    puzzle.lookup_min(intervals.into_iter().cloned())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 46);
    }
}
