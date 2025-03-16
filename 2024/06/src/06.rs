use std::collections::HashSet;

use derive_more::{Deref, DerefMut, FromStr};
use direction::Direction;
use game_grid::Grid;
use position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Cell {
    #[default]
    Empty,
    Wall,
    Guard(Direction),
}

#[derive(Debug)]
struct ParseCellError;

impl TryFrom<char> for Cell {
    type Error = ParseCellError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            _ => Self::Guard(value.try_into().map_err(|_| ParseCellError)?),
        })
    }
}

#[derive(FromStr, Deref, DerefMut, Clone)]
struct Map(Grid<Cell>);

#[derive(Hash, PartialEq, Eq, Clone)]
struct GuardState {
    pos: Position,
    dir: Direction,
}

impl GuardState {
    fn new(map: &Map) -> GuardState {
        map.iter::<Position>()
            .find_map(|(pos, cell)| {
                if let Cell::Guard(dir) = cell {
                    Some(GuardState { pos, dir })
                } else {
                    None
                }
            })
            .unwrap()
    }
    fn step(&mut self, mut command: impl FnMut(&GuardState) -> Command) -> bool {
        match command(&self) {
            Command::Forward => self.pos += self.dir,
            Command::Right => self.dir = self.dir.rotr(),
            Command::Halt => return false,
        };
        true
    }
    fn march(&mut self, mut command: impl FnMut(&GuardState) -> Command) -> MarchResult {
        let mut previous_states: HashSet<GuardState> = HashSet::new();
        loop {
            if previous_states.contains(&self) {
                return MarchResult::Loop;
            }
            previous_states.insert(self.clone());
            if !self.step(&mut command) {
                return MarchResult::Halt;
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum MarchResult {
    Halt,
    Loop,
}

enum Command {
    Forward,
    Right,
    Halt,
}

fn guard_algorithm(map: &Map) -> impl Fn(&GuardState) -> Command {
    |state| {
        if !map.is_in_bounds(state.pos + state.dir) {
            return Command::Halt;
        }
        if map[state.pos + state.dir] == Cell::Wall {
            return Command::Right;
        }
        return Command::Forward;
    }
}

fn part1(input: &str) -> usize {
    let map = input.parse().expect("Parse failed.");
    let mut visited = HashSet::<Position>::new();
    let mut state = GuardState::new(&map);
    let algorithm = guard_algorithm(&map);
    state.march(|state| {
        visited.insert(state.pos);
        algorithm(state)
    });
    visited.len()
}

fn part2(input: &str) -> usize {
    let map = input.parse().expect("Parse failed.");
    let mut ghost = GuardState::new(&map);
    let start = ghost.pos;
    let algorithm = guard_algorithm(&map);
    let mut count = 0usize;
    ghost.march(|state| {
        let mut next_state = state.clone();
        if next_state.step(&algorithm) && next_state.pos != start {
            let obstacle = next_state.pos;
            let mut state = state.clone();
            if state.march(|state| {
                if state.pos + state.dir == obstacle {
                    Command::Right
                } else {
                    algorithm(&state)
                }
            }) == MarchResult::Loop
            {
                count += 1;
            }
        }
        algorithm(state)
    });
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
