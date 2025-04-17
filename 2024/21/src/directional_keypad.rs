use crate::direction::Direction;
use crate::keypad::Keypad;
use enum_iterator::Sequence;
use parse_display::Display;

/* Keypad layout
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct DirectionalKeypad;

#[derive(Clone, Copy, Display, PartialEq, Eq, Hash, Debug, Default, Sequence)]
pub enum Key {
    #[display("{0}")]
    Move(Direction),
    #[default]
    A,
}

impl From<Direction> for Key {
    fn from(direction: Direction) -> Self {
        Key::Move(direction)
    }
}

impl Keypad for DirectionalKeypad {
    type Key = Key;

    fn pos(&self, key: Self::Key) -> (usize, usize) {
        use Direction::*;
        use Key::*;
        match key {
            Move(U) => (0, 1),
            A => (0, 2),
            Move(L) => (1, 0),
            Move(D) => (1, 1),
            Move(R) => (1, 2),
        }
    }

    fn at(&self, pos: (usize, usize)) -> Option<Self::Key> {
        use Direction::*;
        use Key::*;
        Some(match pos {
            (0, 1) => Move(U),
            (0, 2) => A,
            (1, 0) => Move(L),
            (1, 1) => Move(D),
            (1, 2) => Move(R),
            _ => return None,
        })
    }
}
