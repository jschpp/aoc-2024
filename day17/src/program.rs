#[derive(Debug, Clone, Copy)]
pub(crate) enum Instruction {
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
pub(crate) struct Register {
    a: u64,
    b: u64,
    c: u64,
}

impl Register {
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        Self { a, b, c }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    reg: Register,
    code: Vec<u64>,
}

impl Program {
    pub fn new(reg: Register, code: Vec<u64>) -> Self {
        Self { reg, code }
    }

    pub fn run(&mut self) -> Vec<u64> {
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

    pub fn set_a(&mut self, a: u64) {
        self.reg.a = a;
    }

    pub fn code(&self) -> Vec<u64> {
        self.code.clone()
    }

    #[inline(always)]
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

pub(crate) fn result_to_string(result: &[u64]) -> String {
    result
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
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
}
