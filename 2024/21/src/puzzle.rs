use crate::costs::Costs;
use crate::numeric_keypad::Key;
use crate::numeric_keypad::NumericKeypad;
use parse_display::{Display, FromStr};
use parse_display_with::formats::delimiter;
use std::fmt::{Display, Write};
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug)]
pub struct Code(Vec<Key>);

impl IntoIterator for Code {
    type IntoIter = <Vec<Key> as IntoIterator>::IntoIter;
    type Item = Key;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Code {
    type IntoIter = std::slice::Iter<'a, Key>;
    type Item = &'a Key;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromStr for Code {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keys = s
            .chars()
            .map(Key::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseError)?;
        if !keys[0..keys.len() - 1]
            .iter()
            .all(|&k| matches!(k, Key::Digit(_)))
            || !matches!(keys[keys.len() - 1], Key::A)
        {
            return Err(ParseError);
        }
        Ok(Code(keys))
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &key in self {
            f.write_char(key.into())?;
        }
        Ok(())
    }
}

impl Code {
    fn numeric_part(&self) -> usize {
        let s: String = self.0[0..self.0.len() - 1]
            .iter()
            .cloned()
            .map(char::from)
            .collect();
        s.parse().expect("invalid code")
    }
    fn solve_with_costs(&self, costs: &dyn Costs<NumericKeypad>) -> usize {
        let mut key = crate::numeric_keypad::Key::A;
        self.into_iter()
            .map(|&next_key| {
                let c = costs.cost(&NumericKeypad, key, next_key);
                key = next_key;
                c + 1
            })
            .sum::<usize>()
            * self.numeric_part()
    }
}

#[derive(Debug, Display, FromStr)]
pub struct Puzzle {
    #[display(with=delimiter("\n"))]
    pub codes: Vec<Code>,
}

impl Puzzle {
    pub fn solve_with_costs(&self, costs: &dyn Costs<NumericKeypad>) -> usize {
        self.codes
            .iter()
            .map(move |code| code.solve_with_costs(costs))
            .sum()
    }
}
