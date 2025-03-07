type Level = u32;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

impl Direction {
    fn safe_pair(&self, x: Option<Level>, y: Level) -> bool {
        x.is_none_or(|x| match self {
            Direction::Increasing => x + 1 <= y && y <= x + 3,
            Direction::Decreasing => y + 1 <= x && x <= y + 3,
        })
    }
}

fn generate(input: &str) -> impl Iterator<Item = Vec<u32>> {
    input.lines().map(|s| {
        s.split_ascii_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .unwrap()
    })
}

struct Config {
    max_skipped: usize,
    direction: Direction,
}

impl Config {
    fn safe_pair(&self, x: Option<Level>, y: Level) -> bool {
        self.direction.safe_pair(x, y)
    }

    fn check_pairs(&self, items: &[Level], previous: Option<Level>) -> bool {
        match items {
            [] => true,
            [item, rest @ ..] => {
                self.safe_pair(previous, *item) && self.check_pairs(rest, Some(*item))
                    || self.max_skipped > 0
                        && Config {
                            max_skipped: self.max_skipped - 1,
                            ..*self
                        }
                        .check_pairs(rest, previous)
            }
        }
    }
}

fn is_safe(levels: &[Level], max_skipped: usize) -> bool {
    [Direction::Increasing, Direction::Decreasing]
        .iter()
        .any(|&direction| {
            Config {
                max_skipped,
                direction,
            }
            .check_pairs(levels, None)
        })
}

fn count_safe(iter: impl Iterator<Item = Vec<Level>>, max_skipped: usize) -> usize {
    iter.filter(|levels| is_safe(levels, max_skipped)).count()
}

fn part1(input: &str) -> usize {
    count_safe(generate(input), 0)
}

fn part2(input: &str) -> usize {
    count_safe(generate(input), 1)
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 2);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
