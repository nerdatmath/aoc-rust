pub trait ByteCode {
    fn new(opcode: u8, operand: u8) -> Self;
    fn opcode(&self) -> u8;
    fn operand(&self) -> u8;
}
