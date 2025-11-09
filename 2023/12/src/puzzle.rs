use std::{num::ParseIntError, str::FromStr};

use memoize::memoize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Cell {
    Unknown,
    Operational,
    Damaged,
}

#[derive(Debug)]
pub struct Record {
    pub springs: Vec<Cell>,
    pub damaged_groups: Vec<usize>,
}

#[derive(Debug)]
pub struct Puzzle(pub Vec<Record>);

#[derive(Debug)]
pub struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Pat {
    Damaged,
    OperationalSection,
}

fn make_pattern(damaged_groups: &[usize]) -> Vec<Pat> {
    damaged_groups
        .into_iter()
        .map(|&size| vec![Pat::Damaged; size])
        .collect::<Vec<_>>()
        .join(&Pat::OperationalSection)
}

impl Record {
    pub fn solution_count(&self) -> usize {
        solution_count_helper(
            false,
            self.springs.clone(),
            make_pattern(&self.damaged_groups),
        )
    }

    pub fn unfold(&self, times: usize) -> Self {
        Record {
            springs: vec![self.springs.clone(); times].join(&Cell::Unknown),
            damaged_groups: vec![self.damaged_groups.clone(); times].concat(),
        }
    }
}

#[memoize]
fn solution_count_helper(in_damaged_section: bool, springs: Vec<Cell>, pattern: Vec<Pat>) -> usize {
    let result = {
        if springs.len() < pattern.len() {
            return 0;
        }
        match &pattern[..] {
            [] => match &springs[..] {
                [] => 1,
                [Cell::Damaged, ..] => 0,
                [Cell::Operational | Cell::Unknown, springs @ ..] => {
                    solution_count_helper(false, springs.into(), pattern)
                }
            },
            [Pat::OperationalSection, pattern_rest @ ..] => match &springs[..] {
                [] | [Cell::Damaged, ..] => 0,
                [Cell::Operational | Cell::Unknown, springs @ ..] => {
                    solution_count_helper(false, springs.into(), pattern_rest.into())
                }
            },
            [Pat::Damaged, pattern_rest @ ..] => match &springs[..] {
                [] => 0,
                [Cell::Operational, ..] if in_damaged_section => 0,
                [Cell::Operational, springs @ ..] => {
                    solution_count_helper(false, springs.into(), pattern)
                }
                [Cell::Damaged, springs @ ..] => {
                    solution_count_helper(true, springs.into(), pattern_rest.into())
                }
                [Cell::Unknown, springs @ ..] if in_damaged_section => {
                    solution_count_helper(true, springs.into(), pattern_rest.into())
                }
                [Cell::Unknown, springs @ ..] => {
                    solution_count_helper(true, springs.into(), pattern_rest.into())
                        + solution_count_helper(false, springs.into(), pattern)
                }
            },
        }
    };
    result
}

impl FromStr for Record {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, damaged_groups) = s.split_once(' ').ok_or(ParseError)?;
        Ok(Record {
            springs: springs
                .chars()
                .map(|s| {
                    Ok(match s {
                        '?' => Cell::Unknown,
                        '.' => Cell::Operational,
                        '#' => Cell::Damaged,
                        _ => return Err(ParseError),
                    })
                })
                .collect::<Result<_, _>>()?,
            damaged_groups: damaged_groups
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle(
            s.lines().map(|s| s.parse()).collect::<Result<_, _>>()?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::Record;

    fn solution_count(s: &str) -> usize {
        s.parse::<Record>().unwrap().solution_count()
    }

    #[test]
    fn test1() {
        assert_eq!(solution_count("???.### 1,1,3"), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(solution_count(".??..??...?##. 1,1,3"), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(solution_count("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(solution_count("????.#...#... 4,1,1"), 1);
    }

    #[test]
    fn test5() {
        assert_eq!(solution_count("????.######..#####. 1,6,5"), 4);
    }

    #[test]
    fn test6() {
        assert_eq!(solution_count("?###???????? 3,2,1"), 10);
    }
}
