use crate::puzzle::{Position, Puzzle};

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let edges: Box<[(Position, Position, usize)]> = puzzle.distances_iter().collect();
    pathfinding::undirected::kruskal::kruskal(&edges)
        .last()
        .map(|(a, b, _)| a.x * b.x)
        .unwrap()
}

#[test]
fn test1() {
    assert_eq!(run(crate::data::EXAMPLE1), 25272);
}
