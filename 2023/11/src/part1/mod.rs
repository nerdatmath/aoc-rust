use crate::*;
use puzzle::Puzzle;
use stone::Stone;

type Stones = Vec<Stone>;

pub fn blink(stones: &Stones) -> Stones {
    stones.into_iter().flat_map(Stone::blink).collect()
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut stones = puzzle.0;
    for _ in 0..25 {
        stones = blink(&stones);
    }
    stones.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use data::EXAMPLE1;
    use data::EXAMPLE2;

    #[test]
    fn test_blink() -> Result<()> {
        let puzzle: Puzzle = EXAMPLE1.parse()?;
        assert_eq!(
            blink(&puzzle.0),
            [1, 2024, 1, 0, 9, 9, 2021976]
                .into_iter()
                .map(Stone)
                .collect::<Stones>()
        );
        Ok(())
    }

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE2), 55312);
    }
}
