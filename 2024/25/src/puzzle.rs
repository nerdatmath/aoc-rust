use std::str::FromStr;
use std::vec;

#[derive(Debug, PartialEq)]
pub struct Puzzle {
    pub locks: Vec<Lock>,
    pub keys: Vec<Key>,
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locks: Vec<Lock> = vec![];
        let mut keys: Vec<Key> = vec![];
        for stanza in s.split("\n\n") {
            if stanza.starts_with(".....\n") {
                keys.push(stanza.parse()?);
            } else if stanza.starts_with("#####\n") {
                locks.push(stanza.parse()?);
            }
        }
        Ok(Self { locks, keys })
    }
}

#[derive(Debug, PartialEq)]
pub struct Lock([usize; 5]);

impl FromStr for Lock {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = [0usize; 5];
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    heights[col] = row;
                }
            }
        }
        Ok(Self(heights))
    }
}

#[derive(Debug, PartialEq)]
pub struct Key([usize; 5]);

impl Key {
    pub fn fits(&self, lock: &Lock) -> bool {
        for (k, l) in self.0.iter().zip(lock.0.iter()) {
            if k + l > 5 {
                return false;
            }
        }
        true
    }
}

impl FromStr for Key {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = [0usize; 5];
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '.' {
                    heights[col] = 5 - row;
                }
            }
        }
        Ok(Self(heights))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(
            EXAMPLE1.parse::<Puzzle>().unwrap(),
            Puzzle {
                locks: vec![Lock([0, 5, 3, 4, 3]), Lock([1, 2, 0, 5, 3])],
                keys: vec![
                    Key([5, 0, 2, 1, 3]),
                    Key([4, 3, 4, 0, 2]),
                    Key([3, 0, 2, 0, 1]),
                ]
            }
        );
    }
}
