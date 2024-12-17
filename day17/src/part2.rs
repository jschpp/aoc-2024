use crate::{parser::parse, program::Program};

pub fn run() {
    let input = include_str!("../input.txt");
    let (_input, prog) = parse(input).unwrap();

    let part2 = part2(&prog);
    println!("part2: {part2}");
}

fn part2(prog: &Program) -> u64 {
    let code = prog.code();
    let mut a: u64 = 0;
    let mut digits_found: usize = 1;
    loop {
        // get result via decompiled formula
        // this is faster then running the program itself
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
        let result = (0..digits_found)
            .map(|x| {
                let a = a >> (3 * x);
                let b = (a & 7) ^ 3;
                let c = a >> ((a & 7) ^ 5);
                (b ^ c) & 7
            })
            .collect::<Vec<_>>();

        // compare the last digits_found digits of the result with the last digits of the code
        if result == code[code.len() - digits_found..code.len()] {
            if digits_found == code.len() {
                // both slices match along the complete len
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

#[allow(dead_code)]
fn part2_emulator(prog: &mut Program) -> u64 {
    let code = prog.code();
    let mut a: u64 = 0;
    let mut digits_found: usize = 1;
    loop {
        prog.set_a(a);
        let result = prog.run();

        // compare the last digits_found digits of the result with the last digits of the code
        if result == code[code.len() - digits_found..code.len()] {
            if digits_found == code.len() {
                // both slices match along the complete len
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
