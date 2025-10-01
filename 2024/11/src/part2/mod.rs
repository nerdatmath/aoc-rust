use crate::*;
use itertools::Itertools;
use puzzle::Puzzle;
use std::collections::HashMap;
use stone::Stone;

type Stones = HashMap<Stone, usize>;

fn blink(stones: &Stones) -> Stones {
    stones
        .iter()
        .flat_map(|(&stone, &count)| stone.blink().into_iter().map(move |stone| (stone, count)))
        .into_grouping_map()
        .sum()
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut stones: Stones = puzzle.0.iter().map(|&stone| (stone, 1usize)).collect();
    for _ in 0..75 {
        stones = blink(&stones);
    }
    stones.into_iter().map(|(_, count)| count).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use data::EXAMPLE1;

    #[test]
    fn test_blink() -> Result<()> {
        let puzzle: Puzzle = EXAMPLE1.parse()?;
        let stones: Stones = puzzle.0.iter().copied().counts();
        assert_eq!(
            blink(&stones),
            Stones::from([
                (Stone(1), 2),
                (Stone(2024), 1),
                (Stone(0), 1),
                (Stone(9), 2),
                (Stone(2021976), 1)
            ])
        );
        Ok(())
    }
}
