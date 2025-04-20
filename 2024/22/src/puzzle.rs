use parse_display::{Display, FromStr};
use parse_display_with::formats::delimiter;

#[derive(Debug, Display, FromStr)]
pub struct Puzzle {
    #[display(with=delimiter("\n"))]
    pub numbers: Vec<u32>,
}
