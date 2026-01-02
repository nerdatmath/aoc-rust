use crate::intervals::Intervals;
use crate::puzzle::Puzzle;

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let intervals = Intervals::new(&puzzle.fresh_ranges);
    intervals
        .starts
        .iter()
        .cloned()
        .zip(intervals.ends.iter().cloned())
        .filter(|&(start, end)| start <= end)
        .fold((0u64, 0u64), |(last_end, sum), (start, end)| {
            (end + 1, sum + end + 1 - start.max(last_end))
        })
        .1
}

#[test]
fn test1() {
    assert_eq!(run(crate::data::EXAMPLE1), 14);
}
