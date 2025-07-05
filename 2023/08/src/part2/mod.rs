use crate::puzzle::{Node, Puzzle};
use std::{collections::HashSet, rc::Rc};

#[derive(Debug)]
struct State {
    puzzle: Rc<Puzzle>,
    pos: usize, // position in the instructions list
    node: Node,
}

impl State {
    fn step(&mut self) {
        let dir = self.puzzle.instructions[self.pos];
        self.pos = (self.pos + 1) % self.puzzle.instructions.len();
        let node = self.puzzle.nodes[&self.node];
        use crate::puzzle::Direction::*;
        self.node = match dir {
            Left => node.0,
            Right => node.1,
        }
    }
}

// This iterator generates the distances to each target node.
impl Iterator for State {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut count = 0usize;
        self.step();
        count += 1;
        while !self.node.is_target() {
            self.step();
            count += 1;
        }
        Some(count)
    }
}

// Find the distances to each target node, ending when we find a loop.
fn find(puzzle: Rc<Puzzle>, start: Node) -> Option<usize> {
    let mut seen = HashSet::<(Node, usize)>::new();
    let mut state = State {
        puzzle,
        pos: 0,
        node: start,
    };
    let distance = state.next().unwrap();
    seen.insert((state.node, state.pos));
    for d in state {
        if d != distance {
            return None;
        }
        if seen.contains(&(state.node, state.pos)) {
            return Some(distance);
        }
        seen.insert((state.node, state.pos));
    }
    panic!("unreached");
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE3;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE3), 6);
    }
}
