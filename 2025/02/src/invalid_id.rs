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
    let digits = n.ilog10() + 1;
    (2..=digits).any(|repeats| {
        digits.is_multiple_of(repeats) && n.is_multiple_of(ones(digits) / ones(digits / repeats))
    })
}
