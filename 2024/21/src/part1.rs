use crate::costs::robot_stack;
use crate::puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle.solve_with_costs(&robot_stack(3))
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test_example1() {
        assert_eq!(run(EXAMPLE1), 126384);
    }
}
