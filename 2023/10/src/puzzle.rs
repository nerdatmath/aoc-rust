use crate::*;
use anyhow::Result;
use direction::Direction;
use pipe::Pipe;
use position::Position;
use std::str::FromStr;
use thiserror::Error;

type Grid = game_grid::Grid<Pipe>;

fn find_start_position(grid: &Grid) -> Option<Position> {
    grid.iter()
        .find_map(|(pos, pipe)| (pipe == Pipe::Start).then_some(pos))
}

fn patch_pipe(grid: &Grid, position: Position) -> Option<Pipe> {
    use Direction::*;
    use Pipe::*;
    let has_direction = |direction: Direction| -> bool {
        let position = position.next(direction);
        let result = grid.is_in_bounds(position) && grid.cell_at(position).has_direction(direction);
        // dbg!("has_direction", position, direction, result);
        result
    };
    Some(
        match (
            has_direction(N),
            has_direction(S),
            has_direction(E),
            has_direction(W),
        ) {
            (true, true, false, false) => NorthSouth,
            (true, false, true, false) => NorthEast,
            (true, false, false, true) => NorthWest,
            (false, true, true, false) => SouthEast,
            (false, true, false, true) => SouthWest,
            (false, false, true, true) => EastWest,
            _ => return None,
        },
    )
}

fn find_starting_direction(grid: &Grid, starting_position: Position) -> Option<Direction> {
    use Direction::*;
    use Pipe::*;
    Some(match patch_pipe(grid, starting_position)? {
        Ground => unreachable!(),
        NorthSouth => N,
        EastWest => E,
        NorthEast => N,
        NorthWest => N,
        SouthWest => S,
        SouthEast => S,
        Start => unreachable!(),
    })
}

#[derive(Debug)]
pub struct Puzzle {
    pub grid: Grid,
    pub starting_position: Position,
    pub starting_direction: Direction,
}

impl Puzzle {
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            grid: &self.grid,
            position: self.starting_position,
            direction: Some(self.starting_direction),
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct State {
    pub position: Position,
    pub direction: Direction,
    pub pipe: Pipe,
}

#[derive(Debug)]
pub struct Iter<'a> {
    grid: &'a Grid,
    position: Position,
    direction: Option<Direction>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let direction = self.direction?;
        let mut pipe = self.grid.cell_at(self.position);
        if pipe == Pipe::Start {
            pipe = patch_pipe(self.grid, self.position).expect("unpatchable starting position");
        }
        let result = State {
            position: self.position,
            direction,
            pipe,
        };
        self.position.step(direction);
        self.direction = self.grid.cell_at(self.position).next_direction(direction);
        // dbg!(self.position, self.direction);
        Some(result)
    }
}

#[derive(Error, Debug)]
#[error("starting position was not found")]
pub struct StartingPositionNotFound;

#[derive(Error, Debug)]
#[error("starting direction was not found")]
pub struct StartingDirectionNotFound;

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid = s.parse()?;
        let starting_position = find_start_position(&grid).ok_or(StartingPositionNotFound)?;
        let starting_direction: Direction =
            find_starting_direction(&grid, starting_position).ok_or(StartingDirectionNotFound)?;
        Ok(Self {
            grid,
            starting_position,
            starting_direction,
        })
    }
}
