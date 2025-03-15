use std::{cmp::Ordering, collections::HashSet};

use derive_more::{Deref, DerefMut};
use parse_display::{Display, FromStr};
use parse_display_with::formats::delimiter;

#[derive(Clone, Copy, Deref, Display, FromStr, Hash, PartialEq, Eq)]
struct Page(u32);

#[derive(Clone, Copy, Hash, PartialEq, Eq, Display, FromStr)]
#[display("{before}|{after}")]
struct Rule {
    before: Page,
    after: Page,
}

#[derive(Clone, DerefMut, Deref, Display, FromStr)]
struct Update(#[display(with = delimiter(","))] Vec<Page>);

impl Update {
    fn middle(&self) -> Page {
        self[self.len() / 2]
    }
}

#[derive(Clone, DerefMut, Deref, Display, FromStr)]
struct Rules(#[display(with=delimiter("\n"))] HashSet<Rule>);

impl Rules {
    fn cmp(&self, &a: &Page, &b: &Page) -> Ordering {
        if self.contains(&Rule {
            before: a,
            after: b,
        }) {
            Ordering::Less
        } else if self.contains(&Rule {
            before: b,
            after: a,
        }) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Clone, DerefMut, Deref, Display, FromStr)]
struct Updates(#[display(with=delimiter("\n"))] Vec<Update>);

#[derive(Display, FromStr)]
#[display("{rules}\n\n{updates}")]
struct Puzzle {
    rules: Rules,
    updates: Updates,
}

fn part1(input: &str) -> u32 {
    let puzzle: Puzzle = input.parse().expect("Parse failed.");
    let p = |u: &&Update| u.is_sorted_by(|a, b| puzzle.rules.cmp(a, b).is_lt());
    puzzle.updates.iter().filter(p).map(|u| *u.middle()).sum()
}

fn part2(input: &str) -> u32 {
    let puzzle: Puzzle = input.parse().expect("Parse failed.");
    let p = |u: &&Update| !u.is_sorted_by(|a, b| puzzle.rules.cmp(a, b).is_lt());
    let sortit = |u: &Update| {
        let mut u = u.clone();
        u.sort_by(|a, b| puzzle.rules.cmp(a, b));
        u
    };
    puzzle
        .updates
        .iter()
        .filter(p)
        .map(sortit)
        .map(|u| *u.middle())
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 123);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
