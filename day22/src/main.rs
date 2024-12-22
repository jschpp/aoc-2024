use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashMap, iter::Iterator};

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

    let part2_result: ((i8, i8, i8, i8), u64) = initial
        .clone()
        .into_par_iter()
        // generate all sequences for 1 starting seed
        .flat_map(part2)
        // fold it all into one hashmap
        .fold(HashMap::new, |mut acc, (k, v)| {
            acc.entry(k).or_insert(v);
            acc
        })
        .reduce_with(|mut m1, m2| {
            for (k, v) in m2 {
                m1.entry(k).and_modify(|e| *e += v).or_insert(v);
            }
            m1
        })
        .expect("at least one sequence should exist")
        .into_iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .expect("there should be a max");

    println!("part2: {} with diffs: {:?}", part2_result.1, part2_result.0);
}

#[derive(Debug, Clone, Copy)]
struct Seed(u64);

impl Iterator for Seed {
    type Item = Seed;

    fn next(&mut self) -> Option<Self::Item> {
        let mut r = self.0 ^ (self.0 << 6);
        r %= 16777216;
        r ^= r >> 5;
        r %= 16777216;
        r ^= r << 11;
        r %= 16777216;
        self.0 = r;
        Some(Seed(r))
    }
}

// old try for part2
// replaced with Differences Iterator and new part2 function
//
// fn part2(initial: Seed) -> HashMap<(i8, i8, i8, i8), i64> {
//     let mut stack: VecDeque<i8> = VecDeque::with_capacity(4);
//     let mut prev_last_digit: i8 = (initial.0 % 10).try_into().unwrap();
//     let mut result: HashMap<(i8, i8, i8, i8), i64> = HashMap::default();
//     for seed in initial.into_iter().take(2000) {
//         let current_last_digit: i8 = (seed.0 % 10).try_into().unwrap();
//         let diff: i8 = current_last_digit - prev_last_digit;
//         prev_last_digit = current_last_digit;
//         stack.push_back(diff);
//         if stack.len() > 4 {
//             stack.pop_front();
//         }
//         if stack.len() == 4 {
//             let k: (i8, i8, i8, i8) = (stack[0], stack[1], stack[2], stack[3]);
//             if !result.contains_key(&k) {
//                 result
//                     .entry(k)
//                     .and_modify(|e| *e = (*e).max(current_last_digit as i64))
//                     .or_insert(current_last_digit as i64);
//             }
//         }
//     }
//     result
// }

struct Differences<Seed> {
    data: Seed,
    last_digit: u8,
}

impl Iterator for Differences<Seed> {
    type Item = (i8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let current_last_digit: u8 = (self.data.0 % 10).try_into().expect("valid since mod 10");
        let result: i8 = current_last_digit as i8 - self.last_digit as i8;
        self.last_digit = current_last_digit;
        self.data = self.data.next().unwrap();
        Some((result, self.last_digit))
    }
}

impl Differences<Seed> {
    fn new(mut s: Seed) -> Self {
        Self {
            data: s.next().unwrap(),
            last_digit: (s.0 % 10).try_into().expect("valid since mod 10"),
        }
    }
}

fn part2(initial: Seed) -> HashMap<(i8, i8, i8, i8), u64> {
    let d = Differences::new(initial);
    d.into_iter()
        .take(2000)
        .tuple_windows()
        .map(|(a, b, c, d)| ((a.0, b.0, c.0, d.0), d.1))
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_insert(v as u64);
            acc
        })
}
