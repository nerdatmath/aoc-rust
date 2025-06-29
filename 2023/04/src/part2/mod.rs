use std::collections::VecDeque;

use crate::puzzle::Puzzle;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut copies: VecDeque<usize> = vec![1; puzzle.cards.len()].into();
    let mut count = 0usize;
    for card in &puzzle.cards {
        let n = copies.pop_front().unwrap();
        let wins = card.wins();
        count += n;
        for copies in copies.range_mut(0..wins) {
            *copies += n;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 30);
    }
}
