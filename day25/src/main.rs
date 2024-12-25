use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{all_consuming, value},
    multi::{count, separated_list1},
    sequence::terminated,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let (_, (locks, keys)) = parse(input).unwrap();
    let part1_result = locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(k, l)| k.fits_in(l))
        .count();
    println!("part1: {part1_result}");
}

#[derive(Debug, Clone, Copy)]
struct Key {
    data: [u8; 5],
}

impl From<[u8; 5]> for Key {
    fn from(value: [u8; 5]) -> Self {
        Self { data: value }
    }
}

impl Key {
    fn fits_in(&self, lock: &Lock) -> bool {
        self.data.iter().zip(lock.data).all(|(k, l)| k + l <= 5)
    }
}

#[derive(Debug, Clone, Copy)]
struct Lock {
    data: [u8; 5],
}

impl From<[u8; 5]> for Lock {
    fn from(value: [u8; 5]) -> Self {
        Self { data: value }
    }
}

#[derive(PartialEq)]
enum ParseType {
    Key,
    Lock,
}

fn parse(input: &str) -> IResult<&str, (Vec<Key>, Vec<Lock>)> {
    let (input, x) =
        all_consuming(separated_list1(line_ending, alt((parse_key, parse_lock))))(input)?;
    let mut keys: Vec<Key> = Vec::with_capacity(x.len());
    let mut locks: Vec<Lock> = Vec::with_capacity(x.len());
    for (typ, data) in x.into_iter() {
        match typ {
            ParseType::Key => keys.push(data.into()),
            ParseType::Lock => locks.push(data.into()),
        }
    }
    Ok((input, (keys, locks)))
}

fn parse_key(input: &str) -> IResult<&str, (ParseType, [u8; 5])> {
    let (input, _is_key) = terminated(tag("....."), line_ending)(input)?;
    let (input, key) = parse_height(input)?;
    let (input, _is_key) = terminated(tag("#####"), line_ending)(input)?;
    Ok((input, (ParseType::Key, key)))
}

fn parse_lock(input: &str) -> IResult<&str, (ParseType, [u8; 5])> {
    let (input, _is_lock) = terminated(tag("#####"), line_ending)(input)?;
    let (input, lock) = parse_height(input)?;
    let (input, _is_lock) = terminated(tag("....."), line_ending)(input)?;
    Ok((input, (ParseType::Lock, lock)))
}

fn parse_height(input: &str) -> IResult<&str, [u8; 5]> {
    let (input, a) = count(
        terminated(
            count(alt((value(1_u8, tag("#")), value(0, tag(".")))), 5),
            line_ending,
        ),
        5,
    )(input)?;
    let mut height_info = [0_u8; 5];
    for v in a.into_iter() {
        for (idx, n) in v.into_iter().enumerate() {
            height_info[idx] += n
        }
    }
    Ok((input, height_info))
}
