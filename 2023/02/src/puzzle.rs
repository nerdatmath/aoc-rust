use enum_map::{Enum, EnumMap};
use parse_display::{Display, FromStr};
use parse_display_with::formats::{delimiter, fmt_from_str, join};
use std::num::ParseIntError;

pub struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

impl From<parse_display::ParseError> for ParseError {
    fn from(_value: parse_display::ParseError) -> Self {
        Self
    }
}

#[derive(Debug, FromStr)]
pub struct Puzzle {
    #[display(with=delimiter("\n"))]
    pub games: Vec<Game>,
}

#[derive(Debug, FromStr)]
#[display("Game {n}: {counts}")]
pub struct Game {
    pub n: usize,
    #[display(with=join(fmt_from_str(counts_from_str), "; "))]
    pub counts: Vec<Counts>,
}

#[derive(Clone, Copy, Debug, Display, Enum, FromStr)]
#[display(style = "lowercase")]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub type Counts = EnumMap<Color, usize>;

#[derive(FromStr)]
#[display("{n} {color}")]
struct CountsEntry {
    n: usize,
    color: Color,
}

fn counts_from_str(s: &str) -> Result<Counts, ParseError> {
    let mut counts = Counts::default();
    for entry in s.split(", ") {
        let entry: CountsEntry = entry.parse()?;
        counts[entry.color] = entry.n;
    }
    Ok(counts)
}
