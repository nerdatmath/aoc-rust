use crate::puzzle::Puzzle;

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle.distance_sum_2d(2)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 374);
    }
}
