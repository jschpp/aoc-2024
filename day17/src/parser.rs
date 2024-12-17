use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

use crate::program::{Program, Register};

fn parse_registers(input: &str) -> IResult<&str, Register> {
    let (input, a) = terminated(preceded(tag("Register A: "), complete::u64), line_ending)(input)?;
    let (input, b) = terminated(preceded(tag("Register B: "), complete::u64), line_ending)(input)?;
    let (input, c) = terminated(preceded(tag("Register C: "), complete::u64), line_ending)(input)?;
    Ok((input, Register::new(a, b, c)))
}

fn parse_code(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("Program: "), separated_list1(tag(","), complete::u64))(input)
}

fn parse_program(input: &str) -> IResult<&str, Program> {
    let (input, reg) = parse_registers(input)?;
    let (input, _) = line_ending(input)?;
    let (input, code) = parse_code(input)?;
    Ok((input, Program::new(reg, code)))
}

pub fn parse(input: &str) -> IResult<&str, Program> {
    all_consuming(parse_program)(input)
}
