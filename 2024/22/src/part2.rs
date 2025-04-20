use crate::puzzle::Puzzle;
use crate::secret::next;
use itertools::Itertools as _;
use std::collections::HashMap;

fn sequence(n: u32) -> impl Iterator<Item = i8> {
    std::iter::successors(Some(n), |&n| Some(next(n)))
        .map(|n| (n % 10) as i8)
        .take(2001)
}

fn sequences(n: u32) -> impl IntoIterator<Item = ([i8; 4], i8)> {
    sequence(n)
        .tuple_windows()
        .map(|(a, b)| (b - a, b))
        .tuple_windows()
        .map(|((d1, _), (d2, _), (d3, _), (d4, p))| ([d1, d2, d3, d4], p))
        .unique_by(|&pair| pair.0)
}

pub fn run(input: &str) -> i64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut seqs: HashMap<[i8; 4], i64> = HashMap::new();
    let mut biggest: i64 = 0;
    for n in puzzle.numbers {
        for (k, v) in sequences(n) {
            *seqs.entry(k).or_default() += v as i64;
            if seqs[&k] > biggest {
                biggest = seqs[&k];
            }
        }
    }
    biggest
}

#[cfg(test)]
pub mod test {
    use super::run;

    const EXAMPLE2: &'static str = include_str!("../data/example2");

    #[test]
    fn test_example2() {
        assert_eq!(run(EXAMPLE2), 23);
    }
}
