mod parse;

type Num = u32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Mul(Num, Num),
    Do,
    Dont,
}

trait Part
where
    Self: Sized,
{
    fn new() -> Self;
    fn value(&mut self, i: Instruction) -> Option<Num>;
    fn run(instructions: impl IntoIterator<Item = Instruction>) -> Num {
        let mut this = Self::new();
        instructions.into_iter().filter_map(|i| this.value(i)).sum()
    }
}

struct Part1;

impl Part for Part1 {
    fn new() -> Self {
        return Self {};
    }
    fn value(&mut self, i: Instruction) -> Option<Num> {
        if let Instruction::Mul(x, y) = i {
            Some(x * y)
        } else {
            None
        }
    }
}

struct Part2 {
    part1: Part1,
    enabled: bool,
}

impl Part for Part2 {
    fn new() -> Self {
        Self {
            part1: Part1 {},
            enabled: true,
        }
    }
    fn value(&mut self, i: Instruction) -> Option<Num> {
        match i {
            Instruction::Do => self.enabled = true,
            Instruction::Dont => self.enabled = false,
            _ if self.enabled => return self.part1.value(i),
            _ => (),
        }
        return None;
    }
}

fn part1(input: &str) -> Num {
    Part1::run(parse::parse(input))
}

fn part2(input: &str) -> Num {
    Part2::run(parse::parse(input))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE1: &'static str = include_str!("../data/example1/input");
    const EXAMPLE2: &'static str = include_str!("../data/example2/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2), 48);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
