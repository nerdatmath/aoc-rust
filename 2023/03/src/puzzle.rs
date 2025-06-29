use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug, Default)]
pub struct Puzzle {
    pub parts: Vec<Part>,
    pub symbols: HashMap<(usize, usize), Symbol>,
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut puzzle = Puzzle::default();
        for (row, line) in s.lines().enumerate() {
            let mut parsing_part = false;
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '0'..='9' => {
                        if !parsing_part {
                            parsing_part = true;
                            puzzle.parts.push(Part {
                                number: 0,
                                row,
                                col,
                                len: 0,
                            });
                        }
                        let part = puzzle.parts.last_mut().unwrap();
                        part.number = part.number * 10 + ch.to_digit(10).unwrap();
                        part.len += 1;
                    }
                    '.' => parsing_part = false,
                    _ => {
                        puzzle.symbols.insert((row, col), Symbol(ch));
                        parsing_part = false;
                    }
                }
            }
        }
        Ok(puzzle)
    }
}

#[derive(Debug)]
pub struct Part {
    pub number: u32,
    pub row: usize,
    pub col: usize,
    pub len: usize,
}

impl Part {
    pub fn adjacent_points(&self) -> impl IntoIterator<Item = (usize, usize)> {
        let min_col = self.col.saturating_sub(1);
        let max_col = self.col + self.len;
        let min_row = self.row.saturating_sub(1);
        let max_row = self.row + 1;
        let top = (min_col..=max_col).map(move |col| (min_row, col));
        let mid = [min_col, max_col].map(|col| (self.row, col));
        let bot = (min_col..=max_col).map(move |col| (max_row, col));
        top.chain(mid).chain(bot)
    }
}

#[derive(Debug)]
pub struct Symbol(char);

impl Symbol {
    pub fn is_gear(&self) -> bool {
        self.0 == '*'
    }
}
