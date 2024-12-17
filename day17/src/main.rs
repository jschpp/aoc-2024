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

impl TryFrom<i128> for Instruction {
    type Error = String;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
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
    a: i128,
    b: i128,
    c: i128,
}

#[derive(Debug, Clone)]
struct Program {
    reg: Register,
    code: Vec<i128>,
}

impl Program {
    fn run(&mut self) -> Vec<i128> {
        let mut result: Vec<i128> = Vec::default();
        let mut ip: usize = 0;
        while ip < self.code.len() {
            let opcode: Instruction = self.code[ip].try_into().expect("valid opcode");
            let operand = self.code[ip + 1];
            match opcode {
                Instruction::Adv => {
                    let rhs = self.decode_combo(operand);
                    self.reg.a /= 1 << rhs;
                }
                Instruction::Bxl => self.reg.b ^= operand,
                Instruction::Bst => {
                    let rhs = self.decode_combo(operand);
                    self.reg.b = rhs % 8;
                }
                Instruction::Jnz => {
                    if self.reg.a != 0 {
                        ip = operand as usize;
                        continue;
                    }
                }
                Instruction::Bxc => self.reg.b ^= self.reg.c,
                Instruction::Out => result.push(self.decode_combo(operand) % 8),
                Instruction::Bdv => {
                    let rhs = self.decode_combo(operand);
                    self.reg.b = self.reg.a / (1 << rhs);
                }
                Instruction::Cdv => {
                    let rhs = self.decode_combo(operand);
                    self.reg.c = self.reg.a / (1 << rhs);
                }
            };
            ip += 2;
        }
        result
    }

    fn decode_combo(&self, combo_operand: i128) -> i128 {
        match combo_operand {
            i128::MIN..0 => unreachable!(),
            0..=3 => combo_operand,
            4 => self.reg.a,
            5 => self.reg.b,
            6 => self.reg.c,
            7.. => unreachable!(),
        }
    }
}
fn result_to_string(result: &[i128]) -> String {
    result
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(prog: &mut Program) -> i128 {
    // Decompilation of my program
    // 2,4   b = A % 8     => B = a % 8
    // 1,5   B ^= 5        => B = (a % 8) ^ 5
    // 7,5   C = A / 1<<B  => C = a / (1 << ((a % 8) ^ 5))
    // 1,6   B ^= 6        => B = ((a % 8) ^ 5) ^ 6 => (a % 8) ^ 3
    // 0,3   A /= 1<<3     => A = A >> 3
    // 4,1   B ^= C        => B = ((a % 8) ^ 3) ^ (a / (1 << ((a % 8) ^ 5)))
    // 5,5   OUT B % 8

    let code = prog.code.clone();
    let mut a: i128 = 0;
    let mut digits_found: usize = 1;
    loop {
        // get result via decompiled formula
        // this is faster then running the program
        // disabled since not needed after all
        // difference between this and running the program:
        //
        // program:
        // real    0m1,399s
        // user    0m1,344s
        // sys     0m0,023s
        //
        // this:
        // real    0m0,650s
        // user    0m0,606s
        // sys     0m0,027s
        //
        let result = (0..digits_found)
            .map(|x| {
                let a = a >> (3 * x);
                (((a % 8) ^ 3) ^ (a / (1 << ((a % 8) ^ 5)))) % 8
            })
            .collect::<Vec<_>>();

        // prog.reg.a = a;
        // let result = prog.run();

        // compare the last digits_found digits of the result with the last digits of the code
        if result == code[code.len() - digits_found..code.len()] {
            // enable for debug output
            // println!(
            //     "a:{a}, len:{digits_found}, result:{}, last_code:{}",
            //     result_to_string(&result),
            //     result_to_string(&code[code.len() - digits_found..code.len()])
            // );

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
    let (input, a) = terminated(preceded(tag("Register A: "), complete::i128), line_ending)(input)?;
    let (input, b) = terminated(preceded(tag("Register B: "), complete::i128), line_ending)(input)?;
    let (input, c) = terminated(preceded(tag("Register C: "), complete::i128), line_ending)(input)?;
    Ok((input, Register { a, b, c }))
}

fn parse_code(input: &str) -> IResult<&str, Vec<i128>> {
    preceded(tag("Program: "), separated_list1(tag(","), complete::i128))(input)
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
