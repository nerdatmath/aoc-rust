use crate::puzzle::{Counts, Game, Puzzle};

fn counts_power(counts: &Counts) -> usize {
    counts.values().product()
}

fn game_minimum_counts(game: &Game) -> Counts {
    Counts::from_fn(|color| {
        game.counts
            .iter()
            .map(|counts| counts[color])
            .max()
            .unwrap()
    })
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .games
        .into_iter()
        .map(|game| counts_power(&game_minimum_counts(&game)))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 2286);
    }
}
