use regex::Regex;

pub fn run() {
    let content = include_str!("../input.txt");
    let res = parse_input(content);
    println!("{:?}", res.iter().map(|i| i.solve()).sum::<i32>())
}

fn parse_input(content: &str) -> Vec<impl Instruction> {
    let mul_regex = Multiply::regex();
    mul_regex
        .captures_iter(content)
        .map(|m| m.extract())
        .map(|(_, [a, b])| Multiply {
            data: vec![a.parse().expect("valid"), b.parse().expect("valid")],
        })
        .collect()
}

#[derive(Debug)]
struct Multiply {
    data: Vec<i32>,
}

impl Instruction for Multiply {
    fn regex() -> Regex {
        Regex::new(r"mul\((\d+),(\d+)\)").expect("valid Regex")
    }

    fn solve(&self) -> i32 {
        self.data.iter().product()
    }
}

trait Instruction {
    fn regex() -> Regex;
    fn solve(&self) -> i32;
}
