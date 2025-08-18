use crate::nodes::Graph;
use crate::puzzle::Puzzle;
use crate::state::State;
use num_integer::lcm;

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let graph = Graph::new();
    graph.add_nodes(&puzzle.nodes);
    puzzle
        .nodes
        .0
        .keys()
        .filter(|name| name.is_source())
        .map(|name| {
            let cnode = &graph.nodes[name];
            let mut state = State::new(&puzzle.instructions, &cnode);
            state.find(|cnode| cnode.is_target()).unwrap()
        })
        .fold(1, lcm)
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
