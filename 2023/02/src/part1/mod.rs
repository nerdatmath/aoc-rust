use crate::puzzle::{Color, Counts, Game, Puzzle};

fn counts_possible(counts: &Counts) -> bool {
    use Color::*;
    counts[Red] <= 12 && counts[Green] <= 13 && counts[Blue] <= 14
}

fn game_possible(game: &Game) -> bool {
    game.counts.iter().all(counts_possible)
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .games
        .into_iter()
        .filter(game_possible)
        .map(|game| game.n)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 8);
    }
}
