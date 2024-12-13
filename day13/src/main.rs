use glam::{dvec2, DVec2};
use nom::{
    bytes::complete::{tag, take, take_until1},
    character::complete::{self, newline},
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let (_input, machines) = parse(input).unwrap();

    let part1: usize = machines.iter().map(|m| m.part1()).sum();
    println!("part1 {part1}");

    let part2: usize = machines.iter().map(|m| m.part2()).sum();
    println!("part1 {part2}");
}

#[derive(Debug)]
struct Machine {
    a: DVec2,
    b: DVec2,
    prize: DVec2,
}

impl Machine {
    fn solve(&self, delta: u64) -> (f64, f64) {
        let prize = self.prize + delta as f64;

        let determinant = self.a.perp_dot(self.b);
        let n1 = prize.perp_dot(self.b) / determinant;
        let n2 = self.a.perp_dot(prize) / determinant;

        (n1, n2)
    }

    fn part1(&self) -> usize {
        let (a, b) = self.solve(0);
        if a.floor() == a && b.floor() == b {
            a as usize * 3 + b as usize
        } else {
            0_usize
        }
    }

    fn part2(&self) -> usize {
        let (a, b) = self.solve(10000000000000);
        if a.trunc() == a && b.trunc() == b {
            a as usize * 3 + b as usize
        } else {
            0_usize
        }
    }
}

fn parse_value(input: &str) -> IResult<&str, f64> {
    preceded(take(2_usize), complete::i32.map(|x| x as f64))(input)
}

fn parse_line(input: &str) -> IResult<&str, DVec2> {
    let (input, _name) = terminated(take_until1(": "), tag(": "))(input)?;
    let (input, x) = separated_pair(parse_value, tag(", "), parse_value)(input)?;
    let (input, _) = opt(newline)(input)?;
    Ok((input, x.into()))
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, a) = parse_line(input)?;
    let (input, b) = parse_line(input)?;
    let (input, prize) = parse_line(input)?;
    let machine = Machine { a, b, prize };
    Ok((input, machine))
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(newline, parse_machine)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(dvec2(94_f64,34_f64), dvec2(22_f64,67_f64), dvec2(8400_f64,5400_f64), (80_f64,40_f64))]
    #[case(dvec2(17_f64,86_f64), dvec2(84_f64,37_f64), dvec2(7870_f64,6450_f64), (38_f64,86_f64))]
    fn test_lin_alg(
        #[case] a: DVec2,
        #[case] b: DVec2,
        #[case] prize: DVec2,
        #[case] r: (f64, f64),
    ) {
        let m = Machine { a, b, prize };
        assert_eq!(m.solve(0), r)
    }

    #[rstest]
    #[case(
        dvec2(94_f64, 34_f64),
        dvec2(22_f64, 67_f64),
        dvec2(8400_f64, 5400_f64),
        280_usize
    )]
    #[case(
        dvec2(17_f64, 86_f64),
        dvec2(84_f64, 37_f64),
        dvec2(7870_f64, 6450_f64),
        200_usize
    )]
    fn test_part1(#[case] a: DVec2, #[case] b: DVec2, #[case] prize: DVec2, #[case] r: usize) {
        let m = Machine { a, b, prize };
        assert_eq!(m.part1(), r)
    }
}
