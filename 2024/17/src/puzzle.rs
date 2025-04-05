use crate::machine;
use parse_display::{Display, FromStr};
use parse_display_with::formats::delimiter;

#[derive(Display, FromStr)]
#[display("{registers}\n\n{program}")]
pub struct Puzzle {
    pub registers: machine::Registers,
    #[display("Program: {}")]
    #[display(with=delimiter(","))]
    pub program: Box<[u8]>,
}
