use crate::puzzle::Puzzle;

fn run_with_factor(input: &str, factor: u64) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle.distance_sum_2d(factor)
}

pub fn run(input: &str) -> u64 {
    run_with_factor(input, 1000000)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run_with_factor(EXAMPLE1, 10), 1030);
    }

    #[test]
    fn test2() {
        assert_eq!(run_with_factor(EXAMPLE1, 100), 8410);
    }
}
