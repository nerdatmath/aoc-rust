use crate::puzzle::{Battery, Puzzle};

fn max_index(batteries: &[Battery]) -> usize {
    (0..batteries.len())
        .max_by_key(|&i| (batteries[i].joltage, batteries.len() - i))
        .unwrap()
}

fn max_joltage(batteries: &[Battery], max_batteries: u32) -> u64 {
    if max_batteries == 0 {
        return 0;
    }
    let i = max_index(&batteries[0..=batteries.len() - max_batteries as usize]);
    batteries[i].joltage * 10u64.pow(max_batteries - 1)
        + max_joltage(&batteries[i + 1..], max_batteries - 1)
}

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .banks
        .iter()
        .map(|bank| max_joltage(&bank.batteries, 12))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 3121910778619);
    }
}
