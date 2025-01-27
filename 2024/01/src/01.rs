aoc::parts!(1, 2);

pub mod bag;

use bag::Bag;
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
    let mut bags = (Bag::new(), Bag::new());
    for x in parse(input) {
        bags.0.add(x.0);
        bags.1.add(x.1);
    }
    let mut sum: usize = 0;
    for (k, n) in bags.0 {
        sum += k as usize * n * bags.1.get(k);
    }
    sum
}

fn parse(input: aoc::Input) -> impl Iterator<Item = (u32, u32)> + use<'_> {
    input.lines().map(|line| line.ints().into())
}
