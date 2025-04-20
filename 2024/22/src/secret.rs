pub fn next(mut n: u32) -> u32 {
    const MASK: u32 = (1 << 24) - 1;
    n ^= n << 6;
    n &= MASK;
    n ^= n >> 5;
    n ^= n << 11;
    n &= MASK;
    n
}

#[test]
fn test_next() {
    assert_eq!(
        std::iter::successors(Some(next(123)), |&n| Some(next(n)))
            .take(10)
            .collect::<Vec<_>>(),
        [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ]
    );
}
