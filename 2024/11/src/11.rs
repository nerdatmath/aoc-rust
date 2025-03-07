use bag::Bag;

#[derive(PartialEq, Eq, Hash)]
struct Stone(usize);

impl std::str::FromStr for Stone {
    type Err = <usize as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Stone(s.parse()?))
    }
}

impl Stone {
    fn blink(&self) -> impl Iterator<Item = Stone> {
        let n = self.0;
        if n == 0 {
            return vec![Stone(1)].into_iter();
        }
        let digits = n.ilog10() + 1;
        if digits % 2 == 0 {
            let modulus = 10usize.pow(digits / 2);
            return vec![Stone(n / modulus), Stone(n % modulus)].into_iter();
        }
        return vec![Stone(n * 2024)].into_iter();
    }
}

struct Stones(Bag<Stone>);

impl std::str::FromStr for Stones {
    type Err = <Stone as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_ascii_whitespace()
            .map(|s| s.parse::<Stone>())
            .collect::<Result<Stones, _>>()?)
    }
}

impl Stones {
    fn iter(&self) -> bag::Iter<Stone> {
        self.0.iter()
    }

    fn blink(&self) -> Self {
        self.iter()
            .flat_map(|(stone, &count)| stone.blink().map(move |stone| (stone, count)))
            .collect()
    }

    fn count(&self) -> usize {
        self.0.count()
    }
}

impl IntoIterator for Stones {
    type Item = (Stone, usize);
    type IntoIter = bag::IntoIter<Stone>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Stone> for Stones {
    fn from_iter<T: IntoIterator<Item = Stone>>(iter: T) -> Self {
        Stones(iter.into_iter().collect())
    }
}

impl FromIterator<(Stone, usize)> for Stones {
    fn from_iter<T: IntoIterator<Item = (Stone, usize)>>(iter: T) -> Self {
        Stones(iter.into_iter().collect())
    }
}

impl FromIterator<Stones> for Stones {
    fn from_iter<T: IntoIterator<Item = Stones>>(iter: T) -> Self {
        Stones(
            iter.into_iter()
                .flat_map(|stones| stones.into_iter())
                .collect(),
        )
    }
}

fn run(input: &str, n: usize) -> usize {
    let mut stones: Stones = input.parse().expect("Parse error.");
    for _ in 0..n {
        stones = stones.blink()
    }
    stones.count()
}

fn part1(input: &str) -> usize {
    run(input, 25)
}

fn part2(input: &str) -> usize {
    run(input, 75)
}

#[cfg(test)]
mod tests {
    use super::part1;

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 55312);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
