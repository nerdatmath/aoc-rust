use std::{
    collections::{BinaryHeap, HashMap},
    iter::{from_fn, zip},
};

fn to_pair<T: Clone>(iter: impl Iterator<Item = T>) -> (T, T) {
    let v: Vec<T> = iter.collect();
    assert!(v.len() == 2);
    (v[0].clone(), v[1].clone())
}

fn generate(input: &str) -> impl Iterator<Item = (u32, u32)> + use<'_> {
    input.lines().map(|s| {
        to_pair(
            s.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap()),
        )
    })
}

fn part1(input: &str) -> u32 {
    let (mut a, mut b): (BinaryHeap<_>, BinaryHeap<_>) = generate(input).collect();
    zip(from_fn(|| a.pop()), from_fn(|| b.pop()))
        .map(|pair| pair.0.abs_diff(pair.1))
        .sum::<u32>()
}

fn part2(input: &str) -> usize {
    let mut map = HashMap::<u32, (usize, usize)>::new();
    let mut sum = 0usize;
    for row in generate(input) {
        let entry = map.entry(row.0).or_default();
        sum += row.0 as usize * entry.1;
        entry.0 += 1;
        let entry = map.entry(row.1).or_default();
        sum += row.1 as usize * entry.0;
        entry.1 += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 31);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
