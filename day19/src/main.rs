use cached::proc_macro::cached;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::{collections::BTreeMap, sync::Arc};

fn main() {
    let input = include_str!("../input.txt");
    let (_input, (patterns, designs)) = parse(input).expect("valid parse");
    let mut lookup_table: BTreeMap<char, Vec<String>> = BTreeMap::default();
    for pattern in patterns {
        let first: char = pattern.chars().next().expect("not empty");
        lookup_table
            .entry(first)
            .and_modify(|x| x.push(pattern.to_owned()))
            .or_insert(vec![pattern.to_owned()]);
    }
    let lookup_table = Arc::new(lookup_table);
    let part1_result: usize = designs
        .iter()
        .filter(|design| lookup((**design).to_string(), lookup_table.clone()) > 0)
        .count();
    println!("part1: {part1_result}");

    let part2_result: usize = designs
        .iter()
        .map(|design| lookup((**design).to_string(), lookup_table.clone()))
        .sum();
    println!("part2: {part2_result}");
}

#[cached]
fn lookup(current: String, lookup_table: Arc<BTreeMap<char, Vec<String>>>) -> usize {
    if current.is_empty() {
        return 1;
    }
    let first: char = current.chars().next().expect("should not be empty");
    if let Some(pattern) = lookup_table.get(&first) {
        pattern
            .iter()
            .filter(|p| p.len() <= current.len())
            .map(|p| {
                if current.starts_with(p) {
                    let current = current.chars().skip(p.len()).collect::<String>();
                    lookup(current, lookup_table.clone())
                } else {
                    0
                }
            })
            .sum()
    } else {
        0
    }
}

fn parse_pattern(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

fn parse_design(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, alpha1)(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    all_consuming(separated_pair(
        parse_pattern,
        many1(line_ending),
        parse_design,
    ))(input)
}
