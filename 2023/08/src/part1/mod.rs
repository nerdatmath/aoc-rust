use crate::nodes::Graph;
use crate::puzzle::Puzzle;
use crate::state::State;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let graph = Graph::new();
    puzzle.add_to_graph(&graph);
    let start = "AAA".parse().unwrap();
    let target = "ZZZ".parse().unwrap();
    let mut state = State::new(&puzzle.instructions, &graph.nodes[&start]);
    state.target_distance(|node| node.name == target)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data;

    #[test]
    fn test1() {
        assert_eq!(run(data::EXAMPLE1), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(run(data::EXAMPLE2), 6);
    }
}
