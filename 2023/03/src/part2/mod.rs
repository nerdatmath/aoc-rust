use crate::puzzle::{Part, Puzzle, Symbol};
use std::collections::HashMap;

fn adjacent_gears(
    part: &Part,
    symbols: &HashMap<(usize, usize), Symbol>,
) -> impl IntoIterator<Item = (usize, usize)> {
    part.adjacent_points()
        .into_iter()
        .filter(|pos| symbols.get(pos).is_some_and(Symbol::is_gear))
}

pub fn run(input: &str) -> u32 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut gears: HashMap<(usize, usize), Vec<&Part>> = <_>::default();
    for part in &puzzle.parts {
        for gear in adjacent_gears(part, &puzzle.symbols) {
            gears.entry(gear).or_default().push(part);
        }
    }
    gears
        .values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts[0].number * parts[1].number)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 467835);
    }
}
