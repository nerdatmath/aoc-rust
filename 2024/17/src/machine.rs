use crate::instruction::{Combo, Instruction, Machine};
use parse_display::{Display, FromStr};

type Register = u64;

#[derive(Clone, Display, FromStr)]
#[display("{a}\n{b}\n{c}")]
pub struct Registers {
    #[display("Register A: {}")]
    pub a: Register,
    #[display("Register B: {}")]
    pub b: Register,
    #[display("Register C: {}")]
    pub c: Register,
}

type Program = [u8];

pub struct Iter<'a> {
    registers: &'a mut Registers,
    program: &'a Program,
    instruction_pointer: usize,
    pending_output: Option<u8>,
}

impl<'a> Machine<Register> for Iter<'a> {
    fn combo(&self, combo: Combo) -> Register {
        use Combo::*;
        match combo {
            Literal(lit) => lit.into(),
            A => self.registers.a,
            B => self.registers.b,
            C => self.registers.c,
        }
    }
    fn adv(&mut self, value: Register) {
        self.registers.a = self.registers.a >> value;
    }
    fn bxl(&mut self, value: u8) {
        self.registers.b ^= Register::from(value);
    }
    fn bst(&mut self, value: Register) {
        self.registers.b = value % 8;
    }
    fn jnz(&mut self, value: u8) {
        if self.registers.a != 0 {
            self.instruction_pointer = value.into();
        }
    }
    fn bxc(&mut self) {
        self.registers.b ^= self.registers.c;
    }
    fn out(&mut self, value: Register) {
        assert!(self.pending_output.is_none());
        let output: u8 = (value % 8).try_into().unwrap();
        self.pending_output = Some(output);
    }
    fn bdv(&mut self, value: Register) {
        self.registers.b = self.registers.a >> value;
    }
    fn cdv(&mut self, value: Register) {
        self.registers.c = self.registers.a >> value;
    }
}

pub fn run_program<'a>(
    registers: &'a mut Registers,
    program: &'a Program,
) -> impl Iterator<Item = u8> {
    Iter {
        registers,
        program,
        instruction_pointer: 0usize,
        pending_output: None,
    }
}

pub fn fetch(program: &Program, instruction_pointer: usize) -> Option<Instruction> {
    if program.len() < instruction_pointer + 2 {
        return None;
    }
    let opcode = program[instruction_pointer];
    let operand = program[instruction_pointer + 1];
    Some((opcode, operand).into())
}

impl<'a> Iterator for Iter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pending_output.is_none() {
            if let Some(instruction) = fetch(self.program, self.instruction_pointer) {
                self.instruction_pointer += 2;
                self.execute(instruction);
            } else {
                return None;
            }
        }
        let out = self.pending_output;
        self.pending_output = None;
        return out;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let mut registers = Registers { a: 0, b: 0, c: 9 };
        let program = [
            2, 6, // bst C
        ];
        let output: Vec<u8> = run_program(&mut registers, &program).collect();
        assert_eq!(output, []);
        assert_eq!(registers.b, 1);
    }

    #[test]
    fn test_2() {
        let mut registers = Registers { a: 10, b: 0, c: 0 };
        let program = [
            5, 0, // out 0
            5, 1, // out 1
            5, 4, // out A
        ];
        let output: Vec<u8> = run_program(&mut registers, &program).collect();
        assert_eq!(output, [0, 1, 2]);
    }

    #[test]
    fn test_3() {
        let mut registers = Registers {
            a: 2024,
            b: 0,
            c: 0,
        };
        let program = [
            0, 1, // adv 1
            5, 4, // out A
            3, 0, // jnz 0
        ];
        let output: Vec<u8> = run_program(&mut registers, &program).collect();
        assert_eq!(output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(registers.a, 0);
    }

    #[test]
    fn test_4() {
        let mut registers = Registers { a: 0, b: 29, c: 0 };
        let program = [
            1, 7, // bxl 7
        ];
        let output: Vec<u8> = run_program(&mut registers, &program).collect();
        assert_eq!(output, []);
        assert_eq!(registers.b, 26);
    }

    #[test]
    fn test_5() {
        let mut registers = Registers {
            a: 0,
            b: 2024,
            c: 43690,
        };
        let program = [
            4, 0, // bxc 0
        ];
        let output: Vec<u8> = run_program(&mut registers, &program).collect();
        assert_eq!(output, []);
        assert_eq!(registers.b, 44354);
    }
}
