use std::{fmt::Debug, iter::Sum};

use num::Zero;
use parse_display::{Display, FromStr};
use parse_display_with::formats::delimiter;

trait Part1Num
where
    Self: Copy + std::ops::Div<Self, Output = Self>,
{
    fn is_multiple_of(self, other: Self) -> bool;
    fn checked_sub(self, other: Self) -> Option<Self>;

    fn unapply_plus(&self, other: Self) -> impl Iterator<Item = Self> {
        self.checked_sub(other).into_iter()
    }

    fn unapply_multiply(&self, other: Self) -> impl Iterator<Item = Self> {
        if self.is_multiple_of(other) {
            Some(self.div(other))
        } else {
            None
        }
        .into_iter()
    }

    fn unapply_part1(self, other: Self) -> impl Iterator<Item = Self> {
        std::iter::empty()
            .chain(self.unapply_plus(other))
            .chain(self.unapply_multiply(other))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

trait Part2Num: Part1Num {
    fn checked_next_power_of_10(self) -> Option<Self>;

    fn unapply_concat(self, other: Self) -> impl Iterator<Item = Self> {
        other
            .checked_next_power_of_10()
            .into_iter()
            .flat_map(|modulus| {
                self.unapply_plus(other)
                    .flat_map(|this| this.unapply_multiply(modulus).collect::<Vec<Self>>())
                    .collect::<Vec<Self>>()
            })
            .collect::<Vec<Self>>()
            .into_iter()
    }

    fn unapply_part2(self, other: Self) -> impl Iterator<Item = Self> {
        std::iter::empty()
            .chain(self.unapply_plus(other))
            .chain(self.unapply_multiply(other))
            .chain(self.unapply_concat(other))
            .collect::<Vec<Self>>()
            .into_iter()
    }
}

#[derive(Display, FromStr)]
#[display("{answer}: {start} {operands}")]
struct Equation<T> {
    answer: T,
    start: T,
    #[display(with=delimiter(" "))]
    operands: Vec<T>,
}

impl<T> Equation<T> {
    fn is_valid<I, Unapply>(&self, unapply: &Unapply) -> bool
    where
        T: Copy + PartialEq,
        I: Iterator<Item = T>,
        Unapply: Fn(T, T) -> I,
    {
        self.operands
            .iter()
            .copied()
            .rfold(vec![self.answer], |results, operand| {
                results
                    .into_iter()
                    .flat_map(|result| unapply(result, operand))
                    .collect()
            })
            .into_iter()
            .any(|result| result == self.start)
    }

    fn value<I, Unapply>(&self, unapply: &Unapply) -> T
    where
        T: Copy + PartialEq + Zero,
        I: Iterator<Item = T>,
        Unapply: Fn(T, T) -> I,
    {
        if self.is_valid(unapply) {
            self.answer
        } else {
            Zero::zero()
        }
    }
}

fn run<T, I, Unapply>(input: &str, unapply: &Unapply) -> T
where
    T: Copy + PartialEq + Debug + Sum + Zero,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    I: Iterator<Item = T>,
    Unapply: Fn(T, T) -> I,
{
    input
        .lines()
        .map(|s| s.parse::<Equation<T>>().expect("Parse failed."))
        .map(|eq| eq.value(unapply))
        .sum()
}

type Num = u64;

impl Part1Num for Num {
    fn is_multiple_of(self, other: Self) -> bool {
        self % other == 0
    }

    fn checked_sub(self, other: Self) -> Option<Self> {
        Num::checked_sub(self, other)
    }
}

impl Part2Num for Num {
    fn checked_next_power_of_10(self) -> Option<Self> {
        (10 as Num).checked_pow(self.checked_ilog10()? + 1)
    }
}

fn part1(input: &str) -> Num {
    run(input, &Num::unapply_part1)
}

fn part2(input: &str) -> Num {
    run(input, &Num::unapply_part2)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 11387);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
