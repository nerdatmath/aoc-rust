use std::cmp::Reverse;

use crate::puzzle::{Position, Puzzle};

pub fn run(input: &str, count: usize) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut edges: Box<[(Position, Position, usize)]> = puzzle.distances_iter().collect();
    let groups: Box<[Vec<Position>]> = edges
        .select_nth_unstable_by_key(count, |&(_, _, dist)| dist)
        .0
        .iter()
        .map(|&(a, b, _)| vec![a, b])
        .collect();
    let mut component_sizes: Vec<Reverse<usize>> =
        pathfinding::undirected::connected_components::components(&groups)
            .into_iter()
            .map(|component| Reverse(component.len()))
            .collect();
    component_sizes.sort();
    component_sizes
        .into_iter()
        .take(3)
        .map(|Reverse(size)| size)
        .product()
}

#[test]
fn test1() {
    assert_eq!(run(crate::data::EXAMPLE1, 10), 40);
}
