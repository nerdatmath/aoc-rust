use crate::puzzle::Puzzle;

pub fn run(_input: &str) -> usize {
    let puzzle: Puzzle = _input.parse().expect("parse failed");
    puzzle
        .locks
        .iter()
        .map(|lock| puzzle.keys.iter().filter(|&key| key.fits(lock)).count())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 3);
    }
}
