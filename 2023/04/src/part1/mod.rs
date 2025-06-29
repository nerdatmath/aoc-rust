use crate::puzzle::{Card, Puzzle};

fn card_worth(card: Card) -> usize {
    let count = card.wins();
    if count == 0 { 0 } else { 1 << (count - 1) }
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle.cards.into_iter().map(card_worth).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 13);
    }
}
