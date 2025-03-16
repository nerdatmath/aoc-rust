use direction::Direction as Dir;
use game_grid::*;
use parse_display::FromStr;
use position::Position as Pos;
use std::collections::HashSet;

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
enum Cell {
    #[cell('#')]
    Wall,
    #[cell('O')]
    Box,
    #[cell('.')]
    #[default]
    Empty,
    #[cell('@')]
    Robot,
    #[cell('[')]
    LBox,
    #[cell(']')]
    RBox,
}

#[derive(Debug, FromStr)]
struct Map(Grid<Cell>);

impl Map {
    fn push(&mut self, pos: HashSet<Pos>, dir: Dir) -> bool {
        if pos.is_empty() {
            return true;
        }
        let mut newpos: HashSet<Pos> = pos
            .iter()
            .map(|pos| pos + dir)
            .filter(|&pos| self.0[pos] != Cell::Empty)
            .collect();
        if newpos.iter().any(|&pos| self.0[pos] == Cell::Wall) {
            return false;
        }
        if dir.is_vertical() {
            newpos = newpos
                .into_iter()
                .flat_map(|pos| match self.0[pos] {
                    Cell::Wall | Cell::Empty => panic!("Can't be!"),
                    Cell::Box | Cell::Robot => vec![pos],
                    Cell::LBox => vec![pos, pos + Dir::E],
                    Cell::RBox => vec![pos + Dir::W, pos],
                })
                .collect();
        }
        if !self.push(newpos, dir) {
            return false;
        }
        for pos in pos {
            let newpos = pos + dir;
            assert_eq!(self.0[newpos], Cell::Empty);
            self.0.set_cell(newpos, self.0[pos]);
            self.0.set_cell(pos, Cell::Empty);
        }
        return true;
    }

    fn result(&self) -> i32 {
        self.0
            .iter()
            .map(|(Pos { x, y }, cell)| match cell {
                Cell::Box | Cell::LBox => y * 100 + x,
                _ => 0,
            })
            .sum()
    }
}

#[derive(Debug, FromStr)]
#[display("{map}\n\n{directions}")]
struct Puzzle {
    map: Map,
    #[display(with=ParseDirections)]
    directions: Vec<Dir>,
}

struct ParseDirections;

impl parse_display::FromStrFormat<Vec<Dir>> for ParseDirections {
    type Err = direction::ParseDirError;
    fn parse(&self, s: &str) -> core::result::Result<Vec<Dir>, Self::Err> {
        s.chars()
            .filter_map(|ch| (ch != '\n').then(|| Dir::try_from(ch)))
            .collect()
    }
}

impl Puzzle {
    fn run(&mut self) -> i32 {
        let mut robot: Pos = self
            .map
            .0
            .iter()
            .find(|(_, cell)| *cell == Cell::Robot)
            .unwrap()
            .0;
        for &dir in &self.directions {
            if self.map.push(HashSet::from([robot]), dir) {
                robot += dir;
            }
        }
        self.map.result()
    }
}

fn part1(input: &str) -> i32 {
    let mut puzzle: Puzzle = input.parse().expect("Parse failed.");
    puzzle.run()
}

fn double(map: Map) -> Map {
    let data: Vec<Cell> = map
        .0
        .cells()
        .flat_map(|cell| match cell {
            Cell::Box => vec![Cell::LBox, Cell::RBox],
            Cell::Robot => vec![Cell::Robot, Cell::Empty],
            &other => vec![other, other],
        })
        .collect();
    Map(Grid::from_slice_exact(map.0.width() * 2, data.as_slice()))
}

fn part2(input: &str) -> i32 {
    let mut puzzle: Puzzle = input.parse().expect("Parse failed.");
    puzzle.map = double(puzzle.map);
    puzzle.run()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SMALL_EXAMPLE: &'static str = include_str!("../data/small_example");
    const LARGE_EXAMPLE: &'static str = include_str!("../data/large_example");

    #[test]
    fn test_part1_small_example() {
        assert_eq!(part1(SMALL_EXAMPLE), 2028);
    }

    #[test]
    fn test_part1_large_example() {
        assert_eq!(part1(LARGE_EXAMPLE), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(LARGE_EXAMPLE), 9021);
    }
}

fn main() {
    let input = include_str!("../data/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
