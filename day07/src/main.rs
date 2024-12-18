use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let (_, test) = parse_input(input).unwrap();
    let part1: u64 = test
        .iter()
        .filter_map(|(result, nums)| part1(*result, &nums[1..nums.len()], nums[0]))
        .sum();
    println!("part1 {part1}");

    let part2: u64 = test
        .iter()
        .filter_map(|(result, nums)| part2(*result, &nums[1..nums.len()], nums[0]))
        .sum();
    println!("part2 {part2}")
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    let (input, result) = terminated(complete::u64, tag(": "))(input)?;
    let (input, nums) = separated_list1(space1, complete::u64)(input)?;
    Ok((input, (result, nums)))
}

fn part1(result: u64, nums: &[u64], acc: u64) -> Option<u64> {
    if nums.is_empty() {
        if acc == result {
            return Some(result);
        } else {
            return None;
        }
    }
    let current_number = nums.first().expect("not empty");
    let nums = &nums[1..nums.len()];
    match (
        part1(result, nums, acc * current_number),
        part1(result, nums, acc + current_number),
    ) {
        (None, None) => None,
        (None, Some(val)) => Some(val),
        (Some(val), None) => Some(val),
        (Some(val), Some(_)) => Some(val),
    }
}

fn part2(result: u64, nums: &[u64], acc: u64) -> Option<u64> {
    if nums.is_empty() {
        if acc == result {
            return Some(result);
        } else {
            return None;
        }
    }
    let current_number = nums.first().expect("not empty");
    let nums = &nums[1..nums.len()];
    [
        part2(result, nums, acc * current_number),
        part2(result, nums, acc + current_number),
        part2(
            result,
            nums,
            format!("{acc}{current_number}")
                .parse()
                .expect("valid number"),
        ),
    ]
    .iter()
    .filter_map(|x| *x)
    .next()
}
