use crate::invalid_id::has_repeated_digits;
use crate::puzzle::Puzzle;

fn sum_of_invalid_ids_in(it: impl Iterator<Item = u64>) -> u64 {
    it.filter(|&n| has_repeated_digits(n, 2)).sum()
}

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    puzzle
        .ranges
        .iter()
        .cloned()
        .map(sum_of_invalid_ids_in)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_invalid_ids_in_range() {
        assert_eq!(sum_of_invalid_ids_in(11..=22), 33);
        assert_eq!(sum_of_invalid_ids_in(95..=115), 99);
        assert_eq!(sum_of_invalid_ids_in(998..=1012), 1010);
        assert_eq!(sum_of_invalid_ids_in(1188511880..=1188511890), 1188511885);
        assert_eq!(sum_of_invalid_ids_in(222220..=222224), 222222);
        assert_eq!(sum_of_invalid_ids_in(1698522..=1698528), 0);
        assert_eq!(sum_of_invalid_ids_in(446443..=446449), 446446);
        assert_eq!(sum_of_invalid_ids_in(38593856..=38593862), 38593859);
        assert_eq!(sum_of_invalid_ids_in(565653..=565659), 0);
        assert_eq!(sum_of_invalid_ids_in(824824821..=824824827), 0);
        assert_eq!(sum_of_invalid_ids_in(2121212118..=2121212124), 0);
    }

    #[test]
    fn test1() {
        assert_eq!(run(crate::data::EXAMPLE1), 1227775554);
    }
}
