fn main() {
    part1();
    part2();
}

fn part1() {
    let result = parse_reports().into_iter().filter(is_safe).count();
    println!("{}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
    NotSet,
}

type Report = Vec<i32>;

fn parse_reports() -> Vec<Report> {
    let input: &str = include_str!("../input.txt");
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("valid number"))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part2() {
    let result = parse_reports()
        .into_iter()
        .filter_map(|report| {
            if is_safe(&report) {
                Some(report)
            } else {
                for i in 0..report.len() {
                    let mut tmp_report = report.clone();
                    tmp_report.remove(i);
                    if is_safe(&tmp_report) {
                        return Some(report);
                    }
                }
                None
            }
        })
        .count();
    println!("{}", result);
}

fn is_safe(report: &Report) -> bool {
    let mut direction = Direction::NotSet;
    let mut previous_number = report[0];
    for current_number in report.iter().skip(1) {
        match compare_number_and_direction(previous_number, *current_number, direction) {
            Ok(val) => {
                direction = val;
                previous_number = *current_number
            }
            Err(_) => return false,
        }
    }
    true
}

fn compare_number_and_direction(
    prev: i32,
    curr: i32,
    curr_dir: Direction,
) -> Result<Direction, ()> {
    if !(1..=3).contains(&prev.abs_diff(curr)) {
        return Err(());
    }
    use std::cmp::Ordering;
    match (curr.cmp(&prev), curr_dir) {
        (Ordering::Less, Direction::Increasing) => Err(()),
        (Ordering::Less, _) => Ok(Direction::Decreasing),
        (Ordering::Equal, _) => Err(()),
        (Ordering::Greater, Direction::Decreasing) => Err(()),
        (Ordering::Greater, _) => Ok(Direction::Increasing),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 2, Direction::NotSet, Ok(Direction::Increasing))]
    #[case(1, 2, Direction::Increasing, Ok(Direction::Increasing))]
    #[case(1, 2, Direction::Decreasing, Err(()))]
    #[case(2, 1, Direction::Decreasing, Ok(Direction::Decreasing))]
    #[case(2, 1, Direction::NotSet, Ok(Direction::Decreasing))]
    #[case(2, 2, Direction::NotSet, Err(()))]
    #[case(2, 2, Direction::Increasing, Err(()))]
    #[case(2, 2, Direction::Decreasing, Err(()))]
    #[case(2, 7, Direction::Decreasing, Err(()))]
    #[case(7, 2, Direction::Decreasing, Err(()))]
    #[case(7, 6, Direction::NotSet, Ok(Direction::Decreasing))]
    #[case(6, 4, Direction::Decreasing, Ok(Direction::Decreasing))]
    #[case(4, 2, Direction::Decreasing, Ok(Direction::Decreasing))]
    #[case(2, 1, Direction::Decreasing, Ok(Direction::Decreasing))]
    fn test_compare_number_and_direction(
        #[case] prev: i32,
        #[case] curr: i32,
        #[case] curr_dir: Direction,
        #[case] expected: Result<Direction, ()>,
    ) {
        assert_eq!(compare_number_and_direction(prev, curr, curr_dir), expected);
    }

    #[rstest]
    #[case(vec![7,6,4,2,1],true)]
    #[case(vec![1,2,7,8,9],false)]
    #[case(vec![9,7,6,2,1],false)]
    #[case(vec![1,3,2,4,5],false)]
    #[case(vec![8,6,4,4,1],false)]
    #[case(vec![1,3,6,7,9],true)]
    #[case(vec![1,2,4,5],true)]
    fn test_is_safe(#[case] report: Report, #[case] expected: bool) {
        assert_eq!(is_safe(&report), expected)
    }
}
