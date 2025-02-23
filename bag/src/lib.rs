use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Bag<T: Eq + Hash>(HashMap<T, usize>);

pub type Iter<'a, T> = std::collections::hash_map::Iter<'a, T, usize>;
pub type IntoIter<T> = std::collections::hash_map::IntoIter<T, usize>;

impl<T: Eq + Hash> Bag<T> {
    pub fn add(&mut self, item: T) {
        self.add_n(item, 1);
    }

    pub fn add_n(&mut self, item: T, n: usize) {
        *self.0.entry(item).or_default() += n;
    }

    pub fn get(&self, item: T) -> usize {
        *self.0.get(&item).unwrap_or(&0)
    }

    pub fn new() -> Bag<T> {
        Default::default()
    }

    pub fn count(&self) -> usize {
        self.0.values().sum()
    }

    pub fn iter(&self) -> Iter<T> {
        self.0.iter()
    }
}

impl<T: Eq + Hash> Extend<(T, usize)> for Bag<T> {
    fn extend<Iter: IntoIterator<Item = (T, usize)>>(&mut self, iter: Iter) {
        for (item, count) in iter {
            self.add_n(item, count);
        }
    }
}

impl<T: Eq + Hash> Extend<T> for Bag<T> {
    fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
        for item in iter {
            self.add(item);
        }
    }
}

impl<T: Eq + Hash> Default for Bag<T> {
    fn default() -> Bag<T> {
        Bag(Default::default())
    }
}

impl<T: Eq + Hash> IntoIterator for Bag<T> {
    type Item = (T, usize);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Eq + Hash> FromIterator<T> for Bag<T> {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        let mut this = Self::new();
        this.extend(iter);
        this
    }
}

impl<T: Eq + Hash> FromIterator<(T, usize)> for Bag<T> {
    fn from_iter<Iter: IntoIterator<Item = (T, usize)>>(iter: Iter) -> Self {
        let mut this = Self::new();
        this.extend(iter);
        this
    }
}
