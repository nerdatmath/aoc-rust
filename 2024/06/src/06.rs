use std::collections::HashSet;

use array2d::{self, Array2D};

trait Mappy {
    fn in_bounds(&self, row: usize, col: usize) -> bool;
    fn blocked(&self, row: usize, col: usize) -> bool;
}

type Map = Array2D<char>;

impl Mappy for Map {
    fn in_bounds(&self, row: usize, col: usize) -> bool {
        self.get(row, col).is_some()
    }

    fn blocked(&self, row: usize, col: usize) -> bool {
        self.get(row, col) == Some(&'#')
    }
}

fn parse(input: &str) -> Map {
    let rows: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    Array2D::from_rows(&rows).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct GuardState {
    pos: (usize, usize),
    dir: Direction,
}

struct ErrOutOfBounds;

impl GuardState {
    fn new(map: &Map) -> GuardState {
        for (i, row) in map.rows_iter().enumerate() {
            for (j, ch) in row.enumerate() {
                return GuardState {
                    pos: (i, j),
                    dir: match ch {
                        '^' => Direction::N,
                        '>' => Direction::E,
                        'v' => Direction::S,
                        '<' => Direction::W,
                        _ => continue,
                    },
                };
            }
        }
        panic!("Guard position not found.")
    }

    fn step(self: &Self, map: &impl Mappy) -> Result<Self, ErrOutOfBounds> {
        let pos = match self.dir {
            Direction::N => (self.pos.0.checked_sub(1).ok_or(ErrOutOfBounds)?, self.pos.1),
            Direction::S => (self.pos.0 + 1, self.pos.1),
            Direction::E => (self.pos.0, self.pos.1 + 1),
            Direction::W => (self.pos.0, self.pos.1.checked_sub(1).ok_or(ErrOutOfBounds)?),
        };
        if !map.in_bounds(pos.0, pos.1) {
            return Err(ErrOutOfBounds);
        }
        Ok(if map.blocked(pos.0, pos.1) {
            Self {
                pos: self.pos,
                dir: match self.dir {
                    Direction::N => Direction::E,
                    Direction::S => Direction::W,
                    Direction::E => Direction::S,
                    Direction::W => Direction::N,
                },
            }
        } else {
            Self { pos, dir: self.dir }
        })
    }
}

enum MarchResult {
    OutOfBounds,
    Loop,
}

fn march(mut state: GuardState, mappy: &impl Mappy) -> (MarchResult, HashSet<(usize, usize)>) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut previous_states: HashSet<GuardState> = HashSet::new();
    loop {
        if previous_states.contains(&state) {
            return (MarchResult::Loop, visited);
        }
        previous_states.insert(state.clone());
        visited.insert(state.pos);
        state = match state.step(mappy) {
            Ok(newstate) => newstate,
            Err(ErrOutOfBounds) => return (MarchResult::OutOfBounds, visited),
        }
    }
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let state = GuardState::new(&map);
    let (_, visited) = march(state, &map);
    visited.len()
}

struct MapWithObstruction<'a> {
    map: &'a Map,
    obstruction: (usize, usize),
}

impl Mappy for MapWithObstruction<'_> {
    fn in_bounds(&self, row: usize, col: usize) -> bool {
        self.map.in_bounds(row, col)
    }

    fn blocked(&self, row: usize, col: usize) -> bool {
        self.obstruction == (row, col) || self.map.blocked(row, col)
    }
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let state = GuardState::new(&map);
    let (_, visited) = march(state.clone(), &map);
    let mut count = 0usize;
    for obstruction in visited {
        if obstruction == state.pos {
            continue;
        }
        if let (MarchResult::Loop, _) = march(
            state.clone(),
            &MapWithObstruction {
                map: &map,
                obstruction,
            },
        ) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
