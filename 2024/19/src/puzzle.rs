use parse_display::{Display, FromStr};
use parse_display_with::formats::delimiter;

#[derive(Clone, Display, FromStr)]
#[display("{towels}\n\n{patterns}")]
pub struct Puzzle {
    #[display(with=delimiter(", "))]
    pub towels: Vec<String>,
    #[display(with=delimiter("\n"))]
    pub patterns: Vec<String>,
}
