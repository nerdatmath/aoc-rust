use direction::Direction;

use crate::position::Position;
use crate::puzzle::Puzzle;

fn shoelace(mut iter: impl Iterator<Item = Position>) -> i32 {
    let start = iter.next().expect("shoelace: iter must not be empty");
    let mut prev = start;
    let mut area = 0i32;
    for pos in iter {
        area += prev.x * pos.y - pos.x * prev.y;
        prev = pos;
    }
    area += prev.x * start.y - start.x * prev.y;
    area / 2
}

fn left_offset(r#in: Direction, out: Direction) -> (i32, i32) {
    use Direction::*;
    match (r#in, out) {
        (N, E) | (E, N) => (-1, -1),
        (N, W) | (W, N) => (-1, 1),
        (S, E) | (E, S) => (1, -1),
        (S, W) | (W, S) => (1, 1),
        (N, N) => (-1, 0),
        (S, S) => (1, 0),
        (E, E) => (0, -1),
        (W, W) => (0, 1),
        _ => unreachable!(),
    }
}

fn left_area(puzzle: &Puzzle) -> i32 {
    shoelace(puzzle.iter().map(|state| {
        state.position * 2
            + (1, 1)
            + left_offset(
                state
                    .pipe
                    .next_direction(state.direction.reverse())
                    .expect("bad pipe / direction combo")
                    .reverse(),
                state.direction,
            )
    })) / 4
}

fn right_area(puzzle: &Puzzle) -> i32 {
    shoelace(puzzle.iter().map(|state| {
        state.position * 2 + (1, 1)
            - left_offset(
                state
                    .pipe
                    .next_direction(state.direction.reverse())
                    .expect("bad pipe / direction combo")
                    .reverse(),
                state.direction,
            )
    })) / 4
}

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    left_area(&puzzle)
        .abs()
        .min(right_area(&puzzle).abs())
        .try_into()
        .expect("abs() returned a negative number")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data;

    #[test]
    fn test1() {
        assert_eq!(run(data::EXAMPLE1), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(run(data::EXAMPLE2), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(run(data::EXAMPLE3), 4);
    }
    #[test]
    fn test4() {
        assert_eq!(run(data::EXAMPLE4), 8);
    }
    #[test]
    fn test5() {
        assert_eq!(run(data::EXAMPLE5), 10);
    }
}
