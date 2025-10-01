pub fn distance_sum(counts: impl IntoIterator<Item = u64>, factor: u64) -> u64 {
    let mut pos: u64 = 0;
    let mut sum: u64 = 0;
    let mut sum_of_pos: u64 = 0;
    let mut count: u64 = 0;
    for c in counts {
        if c == 0 {
            pos += factor;
            continue;
        }
        pos += 1;
        sum_of_pos += pos * c;
        sum += pos * (count * 2 + c - 1) * c;
        count += c;
    }
    sum - sum_of_pos * (count - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance_sum_1() {
        assert_eq!(distance_sum([2, 1, 0, 1], 2), 13);
    }

    #[test]
    fn test_distance_sum_2() {
        assert_eq!(distance_sum([1, 0, 1, 2], 2), 13);
    }
}
