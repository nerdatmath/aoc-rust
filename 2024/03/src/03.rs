aoc::parts!(1, 2);
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

fn part_1(input: aoc::Input) -> impl ToString {
    parse::parse(input.raw())
        .iter()
        .fold(State::new(true), run)
        .sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    parse::parse(input.raw())
        .iter()
        .fold(State::new(false), run)
        .sum
}
