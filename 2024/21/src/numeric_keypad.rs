use crate::keypad::Keypad;
use enum_iterator::Sequence;
use parse_display::Display;

/* Keypad layout
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct NumericKeypad;

impl Keypad for NumericKeypad {
    type Key = Key;

    fn pos(&self, key: Self::Key) -> (usize, usize) {
        use Key::*;
        match key {
            Digit(d @ (1..=9)) => (2 - (d as usize - 1) / 3, (d as usize - 1) % 3),
            Digit(0) => (3, 1),
            A => (3, 2),
            _ => panic!("invalid key"),
        }
    }

    fn at(&self, pos: (usize, usize)) -> Option<Self::Key> {
        use Key::*;
        Some(match pos {
            (row @ 0..3, col @ 0..3) => Digit(((2 - row) * 3 + col + 1) as u8),
            (3, 1) => Digit(0),
            (3, 2) => A,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug, Default)]
pub enum Key {
    #[display("{0}")]
    Digit(u8),
    #[default]
    A,
}

impl Sequence for Key {
    const CARDINALITY: usize = 11;

    fn next(&self) -> Option<Self> {
        match self {
            Key::Digit(9) => Some(Key::A),
            Key::Digit(d @ 0..9) => Some(Key::Digit(d + 1)),
            Key::A => None,
            _ => panic!("invalid value"),
        }
    }

    fn previous(&self) -> Option<Self> {
        match self {
            Key::Digit(0) => None,
            Key::Digit(d @ 1..=9) => Some(Key::Digit(d - 1)),
            Key::A => Some(Key::Digit(9)),
            _ => panic!("invalid value"),
        }
    }

    fn first() -> Option<Self> {
        Some(Key::Digit(0))
    }

    fn last() -> Option<Self> {
        Some(Key::A)
    }
}

#[derive(Debug)]
pub struct ParseError;

impl TryFrom<char> for Key {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0'..='9' => Key::Digit(value.to_digit(10).unwrap().try_into().unwrap()),
            'A' => Key::A,
            _ => return Err(ParseError),
        })
    }
}

impl From<Key> for char {
    fn from(value: Key) -> Self {
        match value {
            Key::Digit(n) => char::from_digit(n.into(), 10).unwrap(),
            Key::A => 'A',
        }
    }
}
