use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

fn main() {
    let input = include_str!("../demo.txt");
    let (_input, (patterns, designs)) = parse(input).expect("valid parse");
    let mut lookup: HashMap<char, Vec<String>> = HashMap::default();
    for pattern in patterns {
        let first: char = pattern.chars().next().expect("not empty");
        lookup
            .entry(first)
            .and_modify(|x| x.push(pattern.to_owned()))
            .or_insert(vec![pattern.to_owned()]);
    }
    let part1_result: usize = part1(&lookup, &designs);
    println!("part1: {part1_result}");

    let part2_result: usize = part2(&lookup, &designs);
    println!("part2: {part2_result}");
}

fn part1(lookup: &HashMap<char, Vec<String>>, designs: &[&str]) -> usize {
    let cache: Arc<RwLock<HashMap<String, Option<usize>>>> =
        Arc::new(RwLock::new(HashMap::default()));
    let r = designs
        .iter()
        .flat_map(|design| test_string(lookup, design, cache.clone()))
        .count();
    // dbg!(cache.read().unwrap().clone());
    r
}

fn part2(lookup: &HashMap<char, Vec<String>>, designs: &[&str]) -> usize {
    let cache: Arc<RwLock<HashMap<String, Option<usize>>>> =
        Arc::new(RwLock::new(HashMap::default()));
    let r = designs
        .iter()
        .flat_map(|design| test_string(lookup, design, cache.clone()))
        .sum();
    dbg!(cache.read().unwrap().clone());
    r
}

fn test_string(
    lookup: &HashMap<char, Vec<String>>,
    design: &str,
    cache: Arc<RwLock<HashMap<String, Option<usize>>>>,
) -> Option<usize> {
    if cache
        .read()
        .expect("reading should work")
        .contains_key(design)
    {
        return *cache
            .read()
            .expect("reading should work")
            .get(design)
            .expect("since we are in here get should return Some");
    }
    let first: char = design.chars().next().expect("not empty");
    let search_result = if let Some(pattern) = lookup.get(&first) {
        pattern
            .iter()
            .flat_map(|p| {
                if *p == design {
                    Some(1)
                } else if p.len() <= design.len() && **p == design[0..p.len()] {
                    if let Some(r) =
                        test_string(lookup, &design[p.len()..design.len()], cache.clone())
                    {
                        Some(1 + r)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .min()
    } else {
        None
    };
    cache
        .write()
        .unwrap()
        .insert(design.to_owned(), 1);
        // .and_modify(|opt_x| {
        //     if let Some(x) = opt_x {
        //         *opt_x = Some(*x + 1);
        //     } else {
        //         *opt_x = Some(1)
        //     }
        // })
        // .or_insert(Some(1))
        // .clone()
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
