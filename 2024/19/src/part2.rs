use crate::puzzle::Puzzle;

fn successors(pattern: &str, substrings: &Vec<String>, i: usize) -> Vec<usize> {
    substrings
        .into_iter()
        .filter_map(|s| {
            if pattern[i..].starts_with(s) {
                Some(i + s.len())
            } else {
                None
            }
        })
        .collect()
}

fn count_paths(pattern: &str, substrings: &Vec<String>) -> usize {
    pathfinding::directed::count_paths::count_paths(
        0,
        |&i| successors(pattern, substrings, i),
        |&i| i == pattern.len(),
    )
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .patterns
        .into_iter()
        .map(|pattern| count_paths(&pattern, &puzzle.towels))
        .sum()
}

#[cfg(test)]
mod test {
    use super::run;

    const EXAMPLE1: &'static str = include_str!("../data/example1");

    #[test]
    fn test_example1() {
        assert_eq!(run(EXAMPLE1), 16);
    }
}
