use std::{iter::zip, str::FromStr};

use lazy_regex::regex_captures;

#[derive(Debug)]
pub struct Puzzle {
    pub races: Vec<Race>,
}

#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    pub fn ways(&self) -> u64 {
        let q = ceil_sqrt(self.time * self.time - 4 * self.distance);
        q - ((self.time ^ q ^ 1) & 1)
    }
}

#[test]
fn test_race_ways() {
    assert_eq!(
        Race {
            time: 7,
            distance: 9,
        }
        .ways(),
        4
    );
    assert_eq!(
        Race {
            time: 15,
            distance: 40,
        }
        .ways(),
        8
    );
    assert_eq!(
        Race {
            time: 30,
            distance: 200,
        }
        .ways(),
        9
    );
    assert_eq!(
        Race {
            time: 71530,
            distance: 940200,
        }
        .ways(),
        71503
    );
}

fn ceil_sqrt(n: u64) -> u64 {
    return (n - 1).isqrt() + 1;
}

#[test]
fn test_ceil_sqrt() {
    for i in 0u64..5 {
        for n in i.pow(2) + 1..=(i + 1).pow(2) {
            assert_eq!(ceil_sqrt(n), i + 1);
        }
    }
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, time, distance) = regex_captures!(
            r#"Time:\s+(\d+(?:\s+\d+)*)\nDistance:\s+(\d+(?:\s+\d+)*)"#,
            s
        )
        .ok_or(ParseError)?;
        let times: Vec<u64> = time
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| ParseError)?;
        let distances: Vec<u64> = distance
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| ParseError)?;
        if times.len() != distances.len() {
            return Err(ParseError);
        }
        Ok(Puzzle {
            races: zip(times, distances)
                .map(|(time, distance)| Race { time, distance })
                .collect(),
        })
    }
}
