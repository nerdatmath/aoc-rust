const DIVISORS_OF: [&'static [u32]; 19] = [
    &[1],
    &[1, 2],
    &[1, 3],
    &[1, 2, 4],
    &[1, 5],
    &[1, 2, 3, 6],
    &[1, 7],
    &[1, 2, 4, 8],
    &[1, 3, 9],
    &[1, 2, 5, 10],
    &[1, 11],
    &[1, 2, 3, 4, 6, 12],
    &[1, 13],
    &[1, 2, 7, 14],
    &[1, 3, 5, 15],
    &[1, 2, 4, 8, 16],
    &[1, 17],
    &[1, 2, 9, 18],
    &[1, 19],
];

fn ones(digits: u32) -> u64 {
    (10u64.pow(digits - 1) - 1) / 9 * 10 + 1
}

pub fn has_repeated_digits(n: u64, repeats: u32) -> bool {
    let digits = n.ilog10() + 1;
    if !digits.is_multiple_of(repeats) {
        return false;
    }
    n.is_multiple_of(ones(digits) / ones(digits / repeats))
}

pub fn is_repeated(n: u64) -> bool {
    DIVISORS_OF[n.ilog10() as usize][1..]
        .into_iter()
        .any(|&repeats| has_repeated_digits(n, repeats))
}
