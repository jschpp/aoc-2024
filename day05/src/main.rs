use std::{cmp::Ordering, collections::HashMap};

use nom::{bytes::complete::tag, character::complete::{self, line_ending}, multi::separated_list1, sequence::separated_pair, IResult};

#[derive(Debug, Default)]
struct SpecialNumberOrdering {
    order: HashMap<(u64, u64), Ordering>,
}

impl SpecialNumberOrdering {
    fn cmp(&self, lhs: u64, rhs: u64) -> Ordering {
        match self.order.get(&(lhs, rhs)) {
            Some(ord) => *ord,
            None => Ordering::Equal,
        }
    }
}

impl From<Vec<(u64, u64)>> for SpecialNumberOrdering {
    fn from(value: Vec<(u64, u64)>) -> Self {
        let mut ord = SpecialNumberOrdering::default();
        for pair in value {
            ord.order.insert((pair.0, pair.1), Ordering::Less);
            ord.order.insert((pair.1, pair.0), Ordering::Greater);
        }
        ord
    }
}

fn parse_ordering(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(complete::u64, tag("|"), complete::u64)(input)
}

fn parse_input(input: &str) -> IResult<&str, (SpecialNumberOrdering, Vec<Vec<u64>>)> {
    let (input, pairs) = separated_list1(line_ending, parse_ordering)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, orders) =
        separated_list1(line_ending, separated_list1(tag(","), complete::u64))(input)?;
    let special_ordering = SpecialNumberOrdering::from(pairs);
    Ok((input, (special_ordering, orders)))
}

fn main() {
    let input = include_str!("../input.txt");
    let (_, (ord, orders)) = parse_input(input).unwrap();
    let middle_sum: u64 = orders
        .iter()
        .flat_map(|a| {
            let mut b = a.clone();
            b.sort_by(|a, b| ord.cmp(*a, *b));
            if *a == b {
                Some(a[a.len() / 2])
            } else {
                None
            }
        })
        .sum();
    println!("part1: {middle_sum}");

    let middle_sum: u64 = orders
        .iter()
        .flat_map(|a| {
            let mut b = a.clone();
            b.sort_by(|a, b| ord.cmp(*a, *b));
            if *a == b {
                None
            } else {
                Some(b[b.len() / 2])
            }
        })
        .sum();
    println!("part2: {middle_sum}");
}
