use crate::puzzle::{Computer, Puzzle};
use itertools::Itertools as _;
use pathfinding::prelude::maximal_cliques_collect;
use std::collections::HashSet;

pub fn run(input: &str) -> String {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let vertices: HashSet<Computer> = puzzle
        .connections
        .iter()
        .flat_map(|&conn| vec![conn.a, conn.b])
        .collect();
    let cliques = maximal_cliques_collect(vertices, &mut |&a, &b| puzzle.connected(a, b));
    let maximal_clique = cliques
        .iter()
        .max_by(|&a, &b| a.len().cmp(&b.len()))
        .unwrap();
    maximal_clique.into_iter().sorted().join(",")
}

#[cfg(test)]
pub mod test {
    use super::run;

    const EXAMPLE1: &'static str = include_str!("../data/example1");

    #[test]
    fn test_example1() {
        assert_eq!(run(EXAMPLE1), "co,de,ka,ta");
    }
}
