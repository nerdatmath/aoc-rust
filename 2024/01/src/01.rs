aoc::parts!(1, 2);

use aoc::Parse;
use std::{
    collections::{BinaryHeap, HashMap},
    iter::{from_fn, zip},
};

fn part_1(input: aoc::Input) -> impl ToString {
    let mut lists: (BinaryHeap<_>, BinaryHeap<_>) = parse(input).unzip();
    zip(from_fn(|| lists.0.pop()), from_fn(|| lists.1.pop()))
        .map(|pair| pair.0.abs_diff(pair.1))
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut map = HashMap::<u32, (usize, usize)>::new();
    let mut sum = 0usize;
    for row in parse(input) {
        let entry = map.entry(row.0).or_default();
        sum += row.0 as usize * entry.1;
        entry.0 += 1;
        let entry = map.entry(row.1).or_default();
        sum += row.1 as usize * entry.0;
        entry.1 += 1;
    }
    sum
}

fn parse(input: aoc::Input) -> impl Iterator<Item = (u32, u32)> + use<'_> {
    input.lines().map(|line| line.ints().into())
}
