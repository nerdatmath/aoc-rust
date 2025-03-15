use super::Instruction;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    combinator::value,
};

pub fn parse(string: &str) -> Vec<Instruction> {
    match parse_instructions(string) {
        Ok((_, instructions)) => instructions,
        Err(_) => panic!("Parse failed"),
    }
}

fn parse_instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
    let (i, v) = nom::multi::many0(parse_instruction).parse(i)?;
    Ok((i, v.iter().flat_map(|opt| opt).cloned().collect()))
}

pub fn parse_instruction(i: &str) -> IResult<&str, Option<Instruction>> {
    alt((
        parse_mul_instruction.map(Some),
        value(Some(Instruction::Do), tag("do()")),
        value(Some(Instruction::Dont), tag("don't()")),
        value(None, anychar),
    ))
    .parse(i)
}

fn parse_mul_instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("mul(")(i)?;
    let (i, x) = u32(i)?;
    let (i, _) = char(',')(i)?;
    let (i, y) = u32(i)?;
    let (i, _) = char(')')(i)?;
    Ok((i, Instruction::Mul(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mul_instruction() {
        let input = "mul(5,4)";
        let result = parse_mul_instruction(input).unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, Instruction::Mul(5, 4));
    }

    #[test]
    fn test_parse_instruction() {
        let input = "mul(5,4)";
        let result = parse_instruction(input).unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1.unwrap(), Instruction::Mul(5, 4));
    }

    #[test]
    fn test_parse_instructions() {
        let input = include_str!("../data/example1/input");
        let result = parse_instructions(input).unwrap();
        assert_eq!(result.0, "");
        assert_eq!(
            result.1,
            vec![
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            ]
        );
    }

    #[test]
    fn test_parse() {
        let input = include_str!("../data/example1/input");
        let result = parse(input);
        assert_eq!(
            result,
            vec![
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            ]
        );
    }
}
