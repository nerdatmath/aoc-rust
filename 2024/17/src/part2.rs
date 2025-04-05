use crate::instruction::Combo;
use crate::instruction::Instruction;
use crate::machine;
use crate::machine::fetch;
use crate::puzzle::Puzzle;

type Program = [u8];

fn run_out(program: &Program, mut instruction_pointer: usize) -> usize {
    let mut len = 0;
    while let Some(instruction) = fetch(program, instruction_pointer) {
        use Instruction::*;
        if let Out(_) = instruction {
            len += 1;
        }
        instruction_pointer += 2;
    }
    len
}

fn bits_of_a_needed(program: &Program, mut output_len: usize) -> (u8, u8) {
    let mut min_bits = 0;
    let mut bits = 0;
    let mut instruction_pointer = 0;
    loop {
        use Combo::*;
        use Instruction::*;
        let instruction = fetch(program, instruction_pointer).expect("end of program");
        instruction_pointer += 2;
        match instruction {
            Adv(Literal(n)) => bits += n,
            Adv(_) => panic!("not implemented"),
            Jnz(0) => {
                // Try where a = 0 at this point.
                if run_out(program, instruction_pointer + 2) == output_len {
                    return (min_bits, bits);
                }
                // That didn't work so a != 0.
                min_bits = bits;
                instruction_pointer = 0;
            }
            Jnz(_) => panic!("not implemented"),
            Out(_) => {
                assert_ne!(output_len, 0, "too much output");
                output_len -= 1;
            }
            _ => (),
        };
    }
}

fn try_one(program: &Program, mut output: &[u8], a: u64) -> usize {
    let mut registers = machine::Registers { a, b: 0, c: 0 };
    let mut matches = 0usize;
    for out in machine::run_program(&mut registers, program) {
        if out == output[0] {
            matches += 1;
        } else {
            matches = 0;
        }
        output = &output[1..];
    }
    (output.is_empty()).then_some(matches).unwrap_or(0)
}

fn find_a(program: &Program, output: &[u8], mut a: u64, matches: usize) -> Option<u64> {
    // precondition: try_one(program, output, starting_a) == matches
    if matches == output.len() {
        return Some(a);
    }
    let (min_bits, max_bits) = bits_of_a_needed(program, output.len() - matches);
    let max_a = a + (1u64 << u64::from(max_bits));
    while a < max_a {
        if try_one(program, output, a) > matches {
            if let Some(a) = find_a(program, output, a, matches + 1) {
                return Some(a);
            }
        }
        a += 1 << min_bits;
    }
    return None;
}

pub fn run(input: &str) -> u64 {
    let puzzle: Puzzle = input.parse().expect("Parse failed.");
    find_a(&puzzle.program, &puzzle.program, 0, 0).expect("not found")
}

#[cfg(test)]
mod test {
    use super::run;

    const EXAMPLE2: &'static str = include_str!("../data/example2");

    #[test]
    fn test_example2() {
        assert_eq!(run(EXAMPLE2), 117440);
    }
}
