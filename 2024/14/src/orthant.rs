use std::{cmp::Ordering, hash::Hash};

use crate::{Scalar, modular::Modular};

pub trait Orthant {
    type Output: Eq + Hash;
    fn orthant(&self) -> Self::Output;
}

impl<T> Orthant for T
where
    T: Modular<Scalar = Scalar> + Into<Scalar> + Clone,
{
    type Output = Ordering;
    fn orthant(&self) -> Ordering {
        (*self).clone().into().cmp(&((<Self>::MODULUS - 1) / 2))
    }
}
