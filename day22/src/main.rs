use rayon::prelude::*;
use std::iter::Iterator;

fn main() {
    let input = include_str!("../input.txt");
    let initial: Vec<Seed> = input
        .lines()
        .map(|x| Seed(x.trim().parse().unwrap()))
        .collect();
    let part1_result: u64 = initial
        .par_iter()
        .map(|x| x.into_iter().nth(1999).unwrap().0)
        .sum();
    println!("part1 {part1_result}");
}

#[derive(Debug, Clone, Copy)]
struct Seed(u64);

impl Iterator for Seed {
    type Item = Seed;

    fn next(&mut self) -> Option<Self::Item> {
        let mut r = (self.0 ^ (self.0 << 6)) % 16777216;
        r ^= r >> 5;
        r %= 16777216;
        r ^= r << 11;
        r %= 16777216;
        self.0 = r;
        Some(Seed(r))
    }
}
