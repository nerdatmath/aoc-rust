use crate::{machine::run_program, puzzle::Puzzle};
use parse_display::Display;
use parse_display_with::formats::delimiter;

#[derive(Default, Display)]
struct Output(#[display(with=delimiter(","))] Vec<u8>);

pub fn run(input: &str) -> String {
    let mut puzzle: Puzzle = input.parse().expect("Parse failed.");
    Output(run_program(&mut puzzle.registers, &puzzle.program).collect()).to_string()
}

#[cfg(test)]
mod test {
    use super::run;

    const EXAMPLE1: &'static str = include_str!("../data/example1");

    #[test]
    fn test_example1() {
        assert_eq!(run(EXAMPLE1), "4,6,3,5,6,3,5,2,1,0");
    }
}
