use crate::card::Hand as BasicHand;
use parse_display::FromStr;
use std::str::FromStr;

#[derive(Debug)]
pub struct Puzzle<Hand = BasicHand> {
    pub games: Vec<Game<Hand>>,
}

#[derive(Clone, Debug, FromStr)]
#[display("{hand} {bid}")]
pub struct Game<Hand = BasicHand> {
    pub hand: Hand,
    pub bid: usize,
}

impl<Hand: Clone + Ord> Puzzle<Hand> {
    pub fn winnings(&self) -> usize {
        let mut sorted_games = self.games.clone();
        sorted_games.sort_by_key(|game| game.hand.clone());
        sorted_games
            .into_iter()
            .enumerate()
            .map(|(i, game)| (i + 1) * game.bid)
            .sum()
    }
}

#[derive(Debug)]
pub struct ParseError;

impl<Hand: FromStr> FromStr for Puzzle<Hand> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            games: s
                .lines()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()
                .map_err(|_| ParseError)?,
        })
    }
}
