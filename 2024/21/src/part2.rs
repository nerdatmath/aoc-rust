use crate::costs::robot_stack;
use crate::puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle.solve_with_costs(&robot_stack(26))
}
