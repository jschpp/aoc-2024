use std::ops::Index;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, char},
    combinator::{map, value},
    multi::{many1, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Multiply(i32, i32),
    Disable,
    Enable,
}

enum State {
    Enabled,
    Disabled,
}

struct Program {
    state: State,
    data: Vec<Instruction>,
}

impl From<Vec<Instruction>> for Program {
    fn from(value: Vec<Instruction>) -> Self {
        Self {
            state: State::Enabled,
            data: value,
        }
    }
}

impl Index<usize> for Program {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            state: State::Enabled,
            data: Vec::default(),
        }
    }

    pub fn run(&mut self) -> i32 {
        let mut result = 0;
        for instruction in self.data.iter() {
            match instruction {
                Instruction::Multiply(a, b) => match self.state {
                    State::Enabled => result += a * b,
                    State::Disabled => {}
                },
                Instruction::Disable => self.state = State::Disabled,
                Instruction::Enable => self.state = State::Enabled,
            }
        }
        result
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run() {
    let content = include_str!("../input.txt");
    let (_, mut res) = parse_with_nom(content).unwrap();
    assert!(res.len() > 0);
    println!("{:?}", res.run());
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("mul"),
            delimited(
                char('('),
                separated_pair(complete::i32, char(','), complete::i32),
                char(')'),
            ),
        ),
        |(a, b)| Instruction::Multiply(a, b),
    )(input)
}

fn parse_nom_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        mul,
        value(Instruction::Enable, tag("do()")),
        value(Instruction::Disable, tag("don't()")),
    ))(input)
}

fn parse_with_nom(input: &str) -> IResult<&str, Program> {
    many1(map(
        many_till(anychar, parse_nom_instruction),
        |(_, ins)| ins,
    ))(input)
    .map(|(next, p)| (next, p.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mul(1,2)", Instruction::Multiply(1, 2), "")]
    #[case("mul(1,2) ", Instruction::Multiply(1, 2), " ")]
    fn test_mul_parser(#[case] input: &str, #[case] expected: Instruction, #[case] next: &str) {
        let (rest, ins) = mul(input).unwrap();
        assert_eq!(next, rest);
        assert_eq!(ins, expected);
    }

    #[rstest]
    #[should_panic]
    #[case(" mul(1,2) ")]
    #[should_panic]
    #[case("mul[1,2]")]
    #[should_panic]
    #[case("ul(1,2)")]
    fn test_mul_parser_panics(#[case] input: &str) {
        let (_rest, _ins) = mul(input).unwrap();
    }

    #[test]
    fn test_parse_demo_input() {
        let demo = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (_, mut res) = parse_with_nom(demo).unwrap();
        assert_eq!(res.len(), 6);
        assert_eq!(res[0], Instruction::Multiply(2, 4));
        assert_eq!(res[1], Instruction::Disable);
        assert_eq!(res[2], Instruction::Multiply(5, 5));
        assert_eq!(res[3], Instruction::Multiply(11, 8));
        assert_eq!(res[4], Instruction::Enable);
        assert_eq!(res[5], Instruction::Multiply(8, 5));
        assert_eq!(48, res.run())
    }
}
