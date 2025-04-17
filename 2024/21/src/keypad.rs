use crate::direction::Direction;
use enum_iterator::Sequence;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait Keypad: Clone + Copy + Eq + Hash + Debug + Default {
    type Key: Copy + Clone + Default + PartialEq + Eq + Hash + Display + Debug + Sequence;

    fn pos(&self, key: Self::Key) -> (usize, usize);
    fn at(&self, pos: (usize, usize)) -> Option<Self::Key>;

    fn next_key(&self, key: Self::Key, direction: Direction) -> Option<Self::Key> {
        self.at(direction.apply(self.pos(key))?)
    }
}
