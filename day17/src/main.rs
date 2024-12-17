use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let (_input, prog) = parse(input).unwrap();
    let part1 = prog.clone().run();
    let part1 = result_to_string(&part1);
    println!("part1: {part1}");

    let part2 = part2(&mut prog.clone());
    println!("part2: {part2}");
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u64> for Instruction {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jnz),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            x => Err(format!("encountered {x}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Register {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Clone)]
struct Program {
    reg: Register,
    code: Vec<u64>,
}

impl Program {
    fn run(&mut self) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::default();
        let mut ip: usize = 0;
        while ip + 1 < self.code.len() {
            let opcode: Instruction = self.code[ip].try_into().expect("valid opcode");
            let operand = self.code[ip + 1];
            match opcode {
                Instruction::Adv => {
                    let rhs = self.decode_combo(operand);
                    self.reg.a >>= rhs;
                }
                Instruction::Bxl => self.reg.b ^= operand,
                Instruction::Bst => {
                    let rhs = self.decode_combo(operand);
                    self.reg.b = rhs & 7;
                }
                Instruction::Jnz => {
                    if self.reg.a != 0 {
                        ip = operand as usize;
                        continue;
                    }
                }
                Instruction::Bxc => self.reg.b ^= self.reg.c,
                Instruction::Out => result.push(self.decode_combo(operand) & 7),
                Instruction::Bdv => {
                    let rhs = self.decode_combo(operand);
                    self.reg.b = self.reg.a >> rhs;
                }
                Instruction::Cdv => {
                    let rhs = self.decode_combo(operand);
                    self.reg.c = self.reg.a >> rhs;
                }
            };
            ip += 2;
        }
        result
    }

    fn decode_combo(&self, combo_operand: u64) -> u64 {
        match combo_operand {
            0..=3 => combo_operand,
            4 => self.reg.a,
            5 => self.reg.b,
            6 => self.reg.c,
            7.. => unreachable!(),
        }
    }
}
fn result_to_string(result: &[u64]) -> String {
    result
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(prog: &mut Program) -> u64 {
    let code = prog.code.clone();
    let mut a: u64 = 0;
    let mut digits_found: usize = 1;
    loop {
        // get result via decompiled formula
        // this is faster then running the program
        // disabled since not needed after all
        // difference between this and running the program:
        //
        // Decompilation of my program
        // 2,4   b = A % 8     => B = A % 8                    => A & 7 since A mod 8 == A & 7
        // 1,5   B ^= 5        => B = (A % 8) ^ 5              => (A & 7) ^ 5
        // 7,5   C = A / 1<<B  => C = A / (1 << ((A % 8) ^ 5)) => A >> ((A & 7) ^ 5)
        // 1,6   B ^= 6        => B = ((A & 7) ^ 5) ^ 6        => (A & 7) ^ 3
        // 0,3   A /= 1<<3     => A = A >> 3
        // 4,1   B ^= C        => B = ((A & 7) ^ 3) ^ (A >> ((A & 7) ^ 5))
        // 5,5   OUT B % 8
        //
        // program:
        // real    0m0,986s
        // user    0m0,952s
        // sys     0m0,027s
        //
        // this:
        // real    0m0,118s
        // user    0m0,101s
        // sys     0m0,017s
        //
        // let result = (0..digits_found)
        //     .map(|x| {
        //         let a = a >> (3 * x);
        //         let b = (a & 7) ^ 3;
        //         let c = a >> ((a & 7) ^ 5);
        //         (b ^ c) & 7
        //     })
        //     .collect::<Vec<_>>();

        prog.reg.a = a;
        let result = prog.run();

        // compare the last digits_found digits of the result with the last digits of the code
        if result == code[code.len() - digits_found..code.len()] {
            if result == code {
                break;
            }
            // since the decompiled code tells us that a is divided by 8 every run the next digit will be found above a * 8
            a <<= 3;
            digits_found += 1;
        } else {
            a += 1;
        }
    }
    a
}

fn parse_registers(input: &str) -> IResult<&str, Register> {
    let (input, a) = terminated(preceded(tag("Register A: "), complete::u64), line_ending)(input)?;
    let (input, b) = terminated(preceded(tag("Register B: "), complete::u64), line_ending)(input)?;
    let (input, c) = terminated(preceded(tag("Register C: "), complete::u64), line_ending)(input)?;
    Ok((input, Register { a, b, c }))
}

fn parse_code(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("Program: "), separated_list1(tag(","), complete::u64))(input)
}

fn parse_program(input: &str) -> IResult<&str, Program> {
    let (input, reg) = parse_registers(input)?;
    let (input, _) = line_ending(input)?;
    let (input, code) = parse_code(input)?;
    Ok((input, Program { reg, code }))
}

fn parse(input: &str) -> IResult<&str, Program> {
    all_consuming(parse_program)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut p = Program {
            reg: Register { a: 0, b: 0, c: 9 },
            code: vec![2, 6],
        };
        p.run();
        assert_eq!(p.reg.b, 1);
    }

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2
    #[test]
    fn test_b() {
        let mut p = Program {
            reg: Register { a: 10, b: 0, c: 0 },
            code: vec![5, 0, 5, 1, 5, 4],
        };
        let out = p.run();
        let out = result_to_string(&out);
        assert_eq!(&out, "0,1,2");
    }

    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn test_c() {
        let mut p = Program {
            reg: Register {
                a: 2024,
                b: 0,
                c: 0,
            },
            code: vec![0, 1, 5, 4, 3, 0],
        };
        let out = p.run();
        let out = result_to_string(&out);
        assert_eq!(p.reg.a, 0);
        assert_eq!(&out, "4,2,5,6,7,7,7,7,3,1,0");
    }

    // If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn test_d() {
        let mut p = Program {
            reg: Register { a: 0, b: 29, c: 0 },
            code: vec![1, 7],
        };
        p.run();
        assert_eq!(p.reg.b, 26);
    }

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    #[test]
    fn test_e() {
        let mut p = Program {
            reg: Register {
                a: 0,
                b: 2024,
                c: 43690,
            },
            code: vec![4, 0],
        };
        p.run();
        assert_eq!(p.reg.b, 44354);
    }

    #[test]
    fn demo_input() {
        let input = include_str!("../demo.txt");
        let (input, mut prog) = parse(input).expect("valid parse");
        assert!(input.is_empty());
        let out = prog.run();
        let out = result_to_string(&out);
        assert_eq!(&out, "4,6,3,5,6,3,5,2,1,0");
    }
}
