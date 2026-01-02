use parse_display::FromStr;
use parse_display_with::formats::join;

#[derive(Debug, FromStr)]
#[display("{ranges}")]
pub struct Puzzle {
    #[display(with=join(range::RangeFormat, ","))]
    pub ranges: Box<[range::Range]>,
}
