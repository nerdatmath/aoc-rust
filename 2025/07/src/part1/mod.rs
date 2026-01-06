use std::collections::HashSet;

use crate::puzzle::Puzzle;

fn splits(puzzle: &Puzzle) -> usize {
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(puzzle.start);
    let mut splits = 0usize;
    for splitters in &puzzle.splitters {
        for beam in beams
            .intersection(&splitters)
            .cloned()
            .collect::<Vec<usize>>()
        {
            splits += 1;
            beams.remove(&beam);
            // The following are safe because we never have splitters in the far left or far right columns.
            beams.insert(beam - 1);
            beams.insert(beam + 1);
        }
    }
    splits
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    splits(&puzzle)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 21);
    }
}
