use crate::puzzle::Puzzle;

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input
        .replace(' ', "")
        .replace(':', ": ")
        .parse()
        .expect("parse failed");
    puzzle.races.into_iter().map(|race| race.ways()).product()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 71503);
    }
}
