use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Bag<T: Eq + Hash>(HashMap<T, usize>);

impl<T: Eq + Hash> Bag<T> {
    #[inline]
    pub fn add(&mut self, item: T) {
        *self.0.entry(item).or_default() += 1;
    }

    #[inline]
    pub fn get(&self, item: T) -> usize {
        *self.0.get(&item).unwrap_or(&0)
    }

    #[inline]
    pub fn new() -> Bag<T> {
        Default::default()
    }
}

impl<T: Eq + Hash> Default for Bag<T> {
    #[inline]
    fn default() -> Bag<T> {
        Bag(Default::default())
    }
}

impl<T: Eq + Hash> IntoIterator for Bag<T> {
    type Item = (T, usize);

    type IntoIter = std::collections::hash_map::IntoIter<T, usize>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
