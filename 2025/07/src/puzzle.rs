use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Default)]
pub struct Puzzle {
    pub start: usize,
    pub splitters: Box<[HashSet<usize>]>,
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let start = lines
            .next()
            .ok_or(ParseError)?
            .find('S')
            .ok_or(ParseError)?;
        let splitters = lines
            .map(|s| {
                s.chars()
                    .enumerate()
                    .filter_map(|(i, ch)| (ch == '^').then_some(i))
                    .collect()
            })
            .collect();
        Ok(Puzzle { start, splitters })
    }
}
