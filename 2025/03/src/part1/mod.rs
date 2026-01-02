use crate::puzzle::{Battery, Puzzle};

fn max_joltage(batteries: &[Battery]) -> u64 {
    (0..batteries.len() - 1)
        .map(|i| {
            batteries[i].joltage * 10
                + batteries[(i + 1)..]
                    .iter()
                    .map(|battery| battery.joltage)
                    .max()
                    .unwrap()
        })
        .max()
        .unwrap()
}

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .banks
        .iter()
        .map(|bank| max_joltage(&bank.batteries))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 357);
    }
}
