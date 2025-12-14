use crate::puzzle::{Direction, Puzzle};

pub fn run(input: &str) -> usize {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut direction = Direction::R;
    let mut position = 50usize;
    let mut count = 0usize;
    for turn in puzzle.turns {
        if direction != turn.direction {
            if position != 0 {
                position = 100 - position;
            }
            direction = turn.direction;
        }
        position += turn.count;
        count += position.div_euclid(100);
        position = position.rem_euclid(100);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 6);
    }
}
