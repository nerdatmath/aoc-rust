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

fn columns(s: &str) -> Option<String> {
    let len = s.find("\n")?;
    let mut v: Vec<String> = vec![String::default(); len];
    for line in s.lines() {
        for (i, ch) in line.chars().enumerate() {
            v[i].push(ch);
        }
    }
    for s in v.iter_mut() {
        *s = s.trim().to_owned();
    }
    Some(v.join("\n"))
}

fn parse_puzzle(s: &str) -> Result<Puzzle, ParseError> {
    let (inputs, operations) = s.rsplit_once("\n").ok_or(ParseError)?;
    Ok(Puzzle {
        problems: columns(inputs)
            .ok_or(ParseError)?
            .split("\n\n")
            .zip(operations.split_whitespace())
            .map(|(inputs, operation)| [inputs, operation].join("\n").parse())
            .collect::<Result<_, _>>()?,
    })
}

pub fn run(input: &str) -> u64 {
    let puzzle = parse_puzzle(input).expect("parse failed");
    puzzle.problems.iter().map(|problem| problem.solve()).sum()
}

#[test]
fn test1() {
    assert_eq!(run(crate::data::EXAMPLE1), 3263827);
}
