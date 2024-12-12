use cached::proc_macro::cached;
use num_traits::Euclid;
use std::collections::HashMap;

fn main() {
    let numbers = include_str!("../input.txt")
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().expect("input should be valid"))
        .collect::<Vec<_>>();
    let initial_stone_counts: HashMap<usize, usize> =
        numbers.iter().map(|x| (*x, 1_usize)).collect();
    let part1 = blink(25, initial_stone_counts.clone());
    println!("part1 - {part1}");
    let part2 = blink(75, initial_stone_counts.clone());
    println!("part2 - {part2}");
}

fn blink(count: usize, mut stone_counts: HashMap<usize, usize>) -> usize {
    for _ in 0..count {
        stone_counts = stone_counts
            .iter()
            .flat_map(|(num, count)| step(*num).iter().map(|x| (*x, *count)).collect::<Vec<_>>())
            .fold(
                HashMap::default(),
                |mut acc: HashMap<usize, usize>, (x, count)| {
                    acc.entry(x).and_modify(|e| *e += count).or_insert(count);
                    acc
                },
            );
    }
    stone_counts.values().sum()
}

#[cached]
fn len(number: usize) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}

#[cached]
fn step(input: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(2);
    let num_digits = len(input);
    match input {
        0 => result.push(1),
        x if num_digits % 2 == 0 => {
            let (left, right) = x.div_rem_euclid(&10usize.pow(num_digits / 2));
            result.push(left);
            result.push(right);
        }
        x => result.push(x * 2024),
    }
    result
}
