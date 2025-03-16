use std::collections::{HashMap, HashSet};

use game_grid::*;
use parse_display::{Display, FromStr};
use pathfinding::directed::dijkstra::{dijkstra, dijkstra_all};

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
enum Cell {
    #[default]
    #[cell('.')]
    Empty,
    #[cell('#')]
    Wall,
    #[cell('S')]
    Start,
    #[cell('E')]
    End,
    #[cell('O')]
    Found,
}

#[derive(Clone, Copy, Debug, Display, GridPosition, PartialEq, Eq, Hash)]
#[display("({x},{y})")]
struct Position {
    x: i32,
    y: i32,
}

impl std::ops::Add<Direction> for &Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        let (dx, dy): (i32, i32) = rhs.into();
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
enum Direction {
    N = 0,
    S = 180,
    E = 90,
    W = 270,
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct InvalidDirectionError;

impl TryFrom<(i32, i32)> for Direction {
    type Error = InvalidDirectionError;

    fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
        Ok(match value {
            (0, -1) => Direction::N,
            (0, 1) => Direction::S,
            (1, 0) => Direction::E,
            (-1, 0) => Direction::W,
            _ => Err(InvalidDirectionError)?,
        })
    }
}

impl Direction {
    fn rotr(&self) -> Self {
        let (dx, dy): (i32, i32) = (*self).into();
        Self::try_from((-dy, dx)).unwrap()
    }
    fn rotl(&self) -> Self {
        let (dx, dy): (i32, i32) = (*self).into();
        Self::try_from((dy, -dx)).unwrap()
    }
    fn reverse(&self) -> Self {
        let (dx, dy): (i32, i32) = (*self).into();
        Self::try_from((-dx, -dy)).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{pos}{dir}")]
struct Node {
    pos: Position,
    dir: Direction,
}

impl Node {
    fn straight(&self) -> Self {
        Node {
            pos: &self.pos + self.dir,
            dir: self.dir,
        }
    }
    fn back(&self) -> Self {
        Node {
            pos: &self.pos + self.dir.reverse(),
            dir: self.dir,
        }
    }
    fn right(&self) -> Self {
        Node {
            pos: self.pos,
            dir: self.dir.rotr(),
        }
    }
    fn left(&self) -> Self {
        Node {
            pos: self.pos,
            dir: self.dir.rotl(),
        }
    }
}

#[derive(Display, FromStr)]
struct Puzzle {
    grid: Grid<Cell>,
}

impl Puzzle {
    fn find(&self, cell: Cell) -> Position {
        self.grid
            .iter::<Position>()
            .find(|&(_, x)| x == cell)
            .unwrap()
            .0
    }

    fn successors(&self, node: Node) -> impl IntoIterator<Item = (Node, usize)> {
        let mut v: Vec<(Node, usize)> = Vec::new();
        if self.grid[node.straight().pos] != Cell::Wall {
            v.push((node.straight(), 1usize));
        }
        v.push((node.right(), 1000usize));
        v.push((node.left(), 1000usize));
        v
    }

    fn parents(&self, node: Node) -> impl IntoIterator<Item = (Node, usize)> {
        let mut v: Vec<(Node, usize)> = Vec::new();
        if self.grid[node.back().pos] != Cell::Wall {
            v.push((node.back(), 1usize));
        }
        v.push((node.right(), 1000usize));
        v.push((node.left(), 1000usize));
        v
    }
}

fn part1(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("Parse failed.");
    let start = Node {
        pos: puzzle.find(Cell::Start),
        dir: Direction::E,
    };
    let (_path, cost) = dijkstra(
        &start,
        |&node| puzzle.successors(node),
        |node| puzzle.grid[node.pos] == Cell::End,
    )
    .expect("No path found.");
    cost
}

fn part2(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("Parse failed.");
    let start = Node {
        pos: puzzle.find(Cell::Start),
        dir: Direction::E,
    };
    let mut costs: HashMap<Node, usize> = dijkstra_all(&start, |&node| puzzle.successors(node))
        .into_iter()
        .map(|(node, (_parent, cost))| (node, cost))
        .collect();
    costs.insert(start, 0);
    let costs = costs;
    let end = puzzle.find(Cell::End);
    let mut visited: HashSet<Position> = HashSet::new();
    let mut frontier: HashMap<Node, usize> =
        [Direction::E, Direction::N, Direction::W, Direction::S]
            .into_iter()
            .filter_map(|dir| {
                let node = Node { pos: end, dir };
                Some((node, *costs.get(&node)?))
            })
            .collect();
    let min_cost = frontier.iter().map(|(&_node, &cost)| cost).min().unwrap();
    frontier.retain(|_node, cost| *cost == min_cost);
    while !frontier.is_empty() {
        for (node, _cost) in &frontier {
            visited.insert(node.pos);
        }
        frontier = frontier
            .into_iter()
            .flat_map(|(node, node_cost)| {
                if node_cost == 0 {
                    return vec![];
                }
                puzzle
                    .parents(node)
                    .into_iter()
                    .filter_map(|(parent, cost)| {
                        (*costs.get(&parent)? == node_cost.checked_sub(cost)?)
                            .then_some((parent, node_cost.checked_sub(cost)?))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE1: &'static str = include_str!("../data/example1");
    const EXAMPLE2: &'static str = include_str!("../data/example2");

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(EXAMPLE1), 7036);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1(EXAMPLE2), 11048);
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(part2(EXAMPLE1), 45);
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(part2(EXAMPLE2), 64);
    }
}

fn main() {
    let input = include_str!("../data/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
