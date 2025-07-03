use bag::Bag;
use lazy_regex::regex_if;
use std::{cmp::Ordering, iter::zip, str::FromStr};

#[derive(Debug)]
pub struct ParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Debug, Hash)]
pub struct Hand([char; 5]);

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand(cards_from_str(s)?))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        cards_cmp(&self.0, &other.0, false)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Hand {}

#[derive(Clone, Debug, Hash)]
pub struct HandWithJokers([char; 5]);

impl FromStr for HandWithJokers {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HandWithJokers(cards_from_str(s)?))
    }
}

impl Ord for HandWithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        cards_cmp(&self.0, &other.0, true)
    }
}

impl PartialOrd for HandWithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandWithJokers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for HandWithJokers {}

fn cards_from_str(s: &str) -> Result<[char; 5], ParseError> {
    regex_if!(r#"^(?<cards>[2-9TJQKA]{5})$"#, s, {
        cards.chars().collect::<Vec<_>>().try_into().unwrap()
    })
    .ok_or(ParseError)
}

fn cards_rank_cmp(a: &[char; 5], b: &[char; 5], jokers: bool) -> Ordering {
    for (a, b) in zip(a, b) {
        if a != b {
            return match (a, b) {
                ('J', _) if jokers => Ordering::Less,
                (_, 'J') if jokers => Ordering::Greater,
                ('A', _) => Ordering::Greater,
                (_, 'A') => Ordering::Less,
                ('K', _) => Ordering::Greater,
                (_, 'K') => Ordering::Less,
                ('Q', _) => Ordering::Greater,
                (_, 'Q') => Ordering::Less,
                ('J', _) => Ordering::Greater,
                (_, 'J') => Ordering::Less,
                ('T', _) => Ordering::Greater,
                (_, 'T') => Ordering::Less,
                _ => a.cmp(&b),
            };
        }
    }
    Ordering::Equal
}

fn cards_hand_type(cards: &[char; 5], jokers: bool) -> HandType {
    let bag: Bag<char> = cards.iter().cloned().collect();
    let joker_count = if jokers { bag.get('J') } else { 0 };
    if joker_count == 5 {
        return FiveOfAKind;
    }
    let mut counts: Vec<usize> = bag
        .iter()
        .filter_map(|(&ch, &count)| (!(jokers && ch == 'J')).then_some(count))
        .collect();
    counts.sort();
    *counts.last_mut().unwrap() += joker_count;
    use HandType::*;
    match counts[..] {
        [5] => FiveOfAKind,
        [1, 4] => FourOfAKind,
        [2, 3] => FullHouse,
        [1, 1, 3] => ThreeOfAKind,
        [1, 2, 2] => TwoPair,
        [1, 1, 1, 2] => OnePair,
        [1, 1, 1, 1, 1] => HighCard,
        _ => unreachable!("counts don't add to 5 - should be impossible"),
    }
}

fn cards_cmp(a: &[char; 5], b: &[char; 5], jokers: bool) -> Ordering {
    cards_hand_type(a, jokers)
        .cmp(&cards_hand_type(b, jokers))
        .then_with(|| cards_rank_cmp(a, b, jokers))
}
