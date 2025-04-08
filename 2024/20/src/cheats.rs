use crate::puzzle::{Coord, Puzzle};
use itertools::Itertools as _;
use pathfinding::{directed::dfs::dfs_reach, matrix::Matrix};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    coord: Coord,
    distance: usize,
}

fn directions(distance: usize) -> Vec<(isize, isize)> {
    (0..distance)
        .map(|x| (x as isize, (distance - x) as isize))
        .flat_map(|(x, y)| [(x, y), (-x, -y)])
        .flat_map(|(x, y)| [(x, y), (y, -x)])
        .collect()
}

fn directions_up_to(max_distance: usize) -> Vec<(isize, isize)> {
    (1..=max_distance)
        .flat_map(|distance| directions(distance))
        .collect()
}

#[test]
fn test_directions() {
    use itertools::Itertools as _;
    for distance in 1..=20 {
        let directions = directions(distance);
        for &(x, y) in &directions {
            assert_eq!(
                (x.abs() as usize) + (y.abs() as usize),
                distance,
                "manhattan_distance({x},{y}) != {distance}"
            )
        }
        assert!(directions.iter().all_unique());
        assert_eq!(directions.len(), distance * 4);
    }
}

fn main_path(puzzle: &Puzzle) -> Vec<Position> {
    dfs_reach(puzzle.start, |&coord| {
        puzzle
            .grid
            .neighbours(coord.vertex())
            .into_iter()
            .map(Coord::from_vertex)
    })
    .enumerate()
    .map(|(distance, coord)| Position { coord, distance })
    .collect()
}

fn distances(puzzle: &Puzzle) -> Matrix<Option<usize>> {
    let mut mat: Matrix<Option<usize>> = Matrix::new(puzzle.grid.height, puzzle.grid.width, None);
    for pos in main_path(&puzzle) {
        mat[(pos.coord.row, pos.coord.col)] = Some(pos.distance);
    }
    mat
}

fn neighbours(
    distances: &Matrix<Option<usize>>,
    max_jump_distance: usize,
    start: Coord,
) -> impl Iterator<Item = Coord> {
    directions_up_to(max_jump_distance)
        .into_iter()
        .filter_map(move |direction| distances.move_in_direction(start.into(), direction))
        .map(Coord::from)
}

fn savings(distances: &Matrix<Option<usize>>, start: Coord, end: Coord) -> Option<usize> {
    let manhattan_distance = end.row.abs_diff(start.row) + end.col.abs_diff(start.col);
    let start_distance = distances[(start.row, start.col)]?;
    let end_distance = distances[(end.row, end.col)]?;
    end_distance.checked_sub(start_distance + manhattan_distance)
}

fn all_savings(
    distances: &Matrix<Option<usize>>,
    max_jump_distance: usize,
) -> impl Iterator<Item = usize> {
    distances.keys().flat_map(move |start| {
        let start = start.into();
        neighbours(distances, max_jump_distance, start)
            .filter_map(move |end| savings(distances, start, end))
    })
}

pub fn summarize_cheats(
    puzzle: &Puzzle,
    max_jump_distance: usize,
    min_savings: usize,
) -> HashMap<usize, usize> {
    all_savings(&distances(puzzle), max_jump_distance)
        .filter(|&savings| savings >= min_savings)
        .counts()
}
