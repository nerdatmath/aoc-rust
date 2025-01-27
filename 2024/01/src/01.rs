aoc::parts!(1, 2);

use std::collections::HashMap;

use aoc::Parse;

fn part_1(input: aoc::Input) -> impl ToString {
    let inp: Vec<(u32, u32)> = parse(input).collect();
    let n = inp.len();
    let mut left: Vec<u32> = inp.iter().map(|&x| x.0).collect();
    left.sort();
    let mut right: Vec<u32> = inp.iter().map(|&x| x.1).collect();
    right.sort();
    let mut sum = 0;
    for i in 0..n {
        sum += left[i].abs_diff(right[i]);
    }
    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut left: HashMap<u32, u32> = HashMap::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    for (l, r) in parse(input) {
        *left.entry(l).or_default() += 1;
        *right.entry(r).or_default() += 1;
    }
    let mut sum = 0;
    for (k, n) in left {
        sum += k * n * right.get(&k).unwrap_or(&0);
    }
    sum
}

fn parse(input: aoc::Input) -> impl Iterator<Item = (u32, u32)> + use<'_> {
    input.lines().map(|line| line.ints().into())
}
