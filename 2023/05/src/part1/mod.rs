use crate::puzzle::Puzzle;

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let intervals: Vec<[u64; 2]> = puzzle.seeds.iter().map(|&source| [source, 1]).collect();
    puzzle.lookup_min(intervals)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 35);
    }
}
