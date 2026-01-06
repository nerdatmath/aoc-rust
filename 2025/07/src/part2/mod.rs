use std::collections::{HashMap, HashSet};

use crate::puzzle::Puzzle;

fn timelines(
    beam: usize,
    row: usize,
    splitters: &[HashSet<usize>],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&count) = memo.get(&(beam, row)) {
        count
    } else {
        let count = if row == splitters.len() {
            1
        } else if splitters[row].contains(&beam) {
            timelines(beam - 1, row + 1, splitters, memo)
                + timelines(beam + 1, row + 1, splitters, memo)
        } else {
            timelines(beam, row + 1, splitters, memo)
        };
        memo.insert((beam, row), count);
        count
    }
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    timelines(puzzle.start, 0, &puzzle.splitters, &mut memo)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 40);
    }
}
