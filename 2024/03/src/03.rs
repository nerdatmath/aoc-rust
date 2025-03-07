mod parse;

type Num = u32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Mul(Num, Num),
    Do,
    Dont,
}

struct State {
    sum: Num,
    enabled: bool,
    part1: bool,
}

impl State {
    fn new(part1: bool) -> Self {
        State {
            sum: 0,
            enabled: true,
            part1,
        }
    }
}

fn run(mut state: State, instruction: &Instruction) -> State {
    match instruction {
        Instruction::Mul(x, y) if state.enabled => state.sum += x * y,
        Instruction::Mul(_, _) => (),
        Instruction::Do => state.enabled = true,
        Instruction::Dont if !state.part1 => state.enabled = false,
        Instruction::Dont => (),
    }
    state
}

fn part1(input: &str) -> Num {
    parse::parse(input).iter().fold(State::new(true), run).sum
}

fn part2(input: &str) -> Num {
    parse::parse(input).iter().fold(State::new(false), run).sum
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
