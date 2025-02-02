aoc::parts!(1, 2);

use aoc::Parse;

type Level = u32;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn safe_pair(x: Level, y: Level) -> Option<Direction> {
    match (x.abs_diff(y), x < y) {
        (1..=3, true) => Some(Direction::Increasing),
        (1..=3, false) => Some(Direction::Decreasing),
        _ => None,
    }
}

fn check_pairs(
    items: &[Level],
    direction: Direction,
    max_skipped: usize,
    previous: Option<Level>,
) -> bool {
    match items {
        [] => true,
        [item, rest @ ..] => {
            previous.is_none_or(|previous| safe_pair(previous, *item) == Some(direction))
                && check_pairs(rest, direction, max_skipped, Some(*item))
                || max_skipped > 0 && check_pairs(rest, direction, max_skipped - 1, previous)
        }
    }
}

fn is_safe(levels: &[Level], max_skipped: usize) -> bool {
    [Direction::Increasing, Direction::Decreasing]
        .iter()
        .cloned()
        .any(|direction| check_pairs(&levels, direction, max_skipped, None))
}

fn count_safe<Iter: Iterator<Item = Vec<Level>>>(iter: Iter, max_skipped: usize) -> usize {
    iter.filter(|levels| is_safe(levels, max_skipped)).count()
}

fn part_1(input: aoc::Input) -> impl ToString {
    count_safe(parse(input), 0)
}

fn part_2(input: aoc::Input) -> impl ToString {
    count_safe(parse(input), 1)
}

fn parse(input: aoc::Input) -> impl Iterator<Item = Vec<Level>> + use<'_> {
    input.lines().map(|line| line.ints_iter().collect())
}
