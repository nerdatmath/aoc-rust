use bytecode::ByteCode;
use bytecode_derive::ByteCode;
use parse_display::{Display, FromStr};

#[derive(Clone, Copy, Display, FromStr, ByteCode, Debug, PartialEq)]
#[display(style = "lowercase")]
#[display("{} {0}")]
pub enum Instruction {
    #[bytecode(opcode = 0)]
    Adv(Combo),
    #[bytecode(opcode = 1)]
    Bxl(u8),
    #[bytecode(opcode = 2)]
    Bst(Combo),
    #[bytecode(opcode = 3)]
    Jnz(u8),
    #[display("{}")]
    #[bytecode(opcode = 4)]
    Bxc,
    #[bytecode(opcode = 5)]
    Out(Combo),
    #[bytecode(opcode = 6)]
    Bdv(Combo),
    #[bytecode(opcode = 7)]
    Cdv(Combo),
}

pub trait Machine<T> {
    fn combo(&self, combo: Combo) -> T;
    fn adv(&mut self, value: T);
    fn bxl(&mut self, value: u8);
    fn bst(&mut self, value: T);
    fn jnz(&mut self, value: u8);
    fn bxc(&mut self);
    fn out(&mut self, value: T);
    fn bdv(&mut self, value: T);
    fn cdv(&mut self, value: T);
    fn execute(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            Adv(combo) => self.adv(self.combo(combo)),
            Bxl(value) => self.bxl(value),
            Bst(combo) => self.bst(self.combo(combo)),
            Jnz(value) => self.jnz(value),
            Bxc => self.bxc(),
            Out(combo) => self.out(self.combo(combo)),
            Bdv(combo) => self.bdv(self.combo(combo)),
            Cdv(combo) => self.cdv(self.combo(combo)),
        }
    }
}

#[derive(Clone, Copy, Display, FromStr, Debug, PartialEq)]
#[display("{}")]
pub enum Combo {
    #[display("{0}")]
    Literal(u8),
    A,
    B,
    C,
}

impl From<u8> for Combo {
    fn from(value: u8) -> Self {
        use Combo::*;
        match value {
            value @ 0..=3 => Literal(value),
            4 => A,
            5 => B,
            6 => C,
            _ => panic!("invalid operand"),
        }
    }
}

impl From<Combo> for u8 {
    fn from(value: Combo) -> Self {
        use Combo::*;
        match value {
            Literal(value) => value,
            A => 4,
            B => 5,
            C => 6,
        }
    }
}

#[cfg(test)]
pub fn assemble(instructions: &[Instruction]) -> Box<[u8]> {
    instructions
        .into_iter()
        .flat_map(|inst| vec![inst.opcode(), inst.operand()].into_iter())
        .collect()
}

#[cfg(test)]
mod test {
    use super::{
        Combo::*,
        Instruction::{self, *},
        assemble,
    };

    #[test]
    fn test_1() {
        assert_eq!(assemble(&[Adv(Literal(0))]), [0u8, 0u8].into());
    }

    #[test]
    fn test_2() {
        assert_eq!("adv 0".parse::<Instruction>().unwrap(), Adv(Literal(0)));
    }
}
