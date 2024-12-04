use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    // part 1
    let mut day01 = Day01::parse(input);
    println!("part 1: {}", day01.solve());

    // part 1
    let day01b = Day01b::parse(input);
    println!("part 2: {}", day01b.solve());
}

#[derive(Debug)]
struct Day01 {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Day01 {
    fn parse(input: &str) -> Self {
        let mut result = Self {
            left: vec![],
            right: vec![],
        };
        for line in input.lines() {
            let tmp: Vec<usize> = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().expect("valid usize"))
                .collect();
            assert!(tmp.len() >= 2);
            result.left.push(tmp[0]);
            result.right.push(tmp[1]);
        }
        result
    }

    fn sort(&mut self) {
        self.right.sort();
        self.left.sort();
    }

    fn get_differences(&self) -> Vec<usize> {
        self.left
            .iter()
            .zip(&self.right)
            .map(|(l, r)| l.abs_diff(*r))
            .collect()
    }

    fn solve(&mut self) -> usize {
        self.sort();
        self.get_differences().iter().sum()
    }
}

#[derive(Debug)]
struct Day01b {
    left: Vec<usize>,
    right: HashMap<usize, usize>,
}

impl Day01b {
    fn parse(input: &str) -> Self {
        let mut result = Self {
            left: vec![],
            right: HashMap::default(),
        };
        for line in input.lines() {
            let tmp: Vec<usize> = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().expect("valid usize"))
                .collect();
            assert!(tmp.len() >= 2);
            result.left.push(tmp[0]);
            result
                .right
                .entry(tmp[1])
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        result
    }

    fn get_differences(&self) -> Vec<usize> {
        self.left
            .iter()
            .map(|l| l * self.right.get(l).unwrap_or(&0))
            .collect()
    }

    fn solve(&self) -> usize {
        self.get_differences().iter().sum()
    }
}
