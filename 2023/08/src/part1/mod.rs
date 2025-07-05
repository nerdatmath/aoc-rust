use crate::puzzle::{Node, Puzzle};

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let start: Node = "AAA".parse().unwrap();
    let target: Node = "ZZZ".parse().unwrap();
    puzzle
        .instructions
        .iter()
        .cycle()
        .scan(start, |node, dir| {
            match dir {
                crate::puzzle::Direction::Left => *node = puzzle.nodes[node].0,
                crate::puzzle::Direction::Right => *node = puzzle.nodes[node].1,
            }
            Some(*node)
        })
        .position(|node| node == target)
        .unwrap()
        + 1
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::{EXAMPLE1, EXAMPLE2};

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(run(EXAMPLE2), 6);
    }
}
