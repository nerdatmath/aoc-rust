use std::collections::HashSet;

use parse_display::FromStr;
use parse_display_with::formats::delimiter;

#[derive(Debug, FromStr)]
#[display("{fresh_ranges}\n\n{available}")]
pub struct Puzzle {
    #[display(with=delimiter("\n"))]
    pub fresh_ranges: Box<[Range]>,
    #[display(with=delimiter("\n"))]
    pub available: HashSet<u64>,
}

#[derive(Debug, FromStr)]
#[display("{start}-{end}")]
pub struct Range {
    pub start: u64,
    pub end: u64,
}
