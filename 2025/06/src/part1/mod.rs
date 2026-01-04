use crate::puzzle::Puzzle;

#[derive(Debug)]
struct ParseError;

impl From<std::num::ParseIntError> for ParseError {
    fn from(_value: std::num::ParseIntError) -> Self {
        ParseError
    }
}

impl From<crate::puzzle::ParseError> for ParseError {
    fn from(_value: crate::puzzle::ParseError) -> Self {
        ParseError
    }
}

fn parse_puzzle(s: &str) -> Result<Puzzle, ParseError> {
    let mut lines = s.lines();
    let mut input_columns: Box<[Vec<&str>]> = lines
        .next()
        .ok_or(ParseError)?
        .split_ascii_whitespace()
        .map(|s| vec![s])
        .collect();
    for line in lines {
        for (inputs, input) in (input_columns.iter_mut()).zip(line.split_ascii_whitespace()) {
            inputs.push(input)
        }
    }
    Ok(Puzzle {
        problems: input_columns
            .into_iter()
            .map(|inputs| inputs.join("\n").parse())
            .collect::<Result<_, _>>()?,
    })
}

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = parse_puzzle(input).expect("parse failed");
    puzzle.problems.iter().map(|problem| problem.solve()).sum()
}

#[test]
fn test1() {
    assert_eq!(run(crate::data::EXAMPLE1), 4277556);
}
