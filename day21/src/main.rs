use itertools::Itertools;
use pathfinding::prelude::*;
use std::{collections::HashMap, fmt::Display};

fn main() {
    let input = include_str!("../input.txt");
    let inputs: Vec<Vec<KeypadButton>> = input.lines().map(parse).collect();

    let part1_result: usize = inputs
        .clone()
        .into_iter()
        .map(|input_s| get_num(&input_s) * part1(input_s)[0].len())
        .sum();
    println!("part1: {part1_result}");

    let part2_result: usize = inputs
        .into_iter()
        .map(|input_s| get_num(&input_s) * part2(input_s)[0].len())
        .sum();
    println!("part2: {part2_result}");
}

fn get_num(s: &[KeypadButton]) -> usize {
    let mut x = 1;
    let mut result: usize = 0;
    for num in s.iter().rev() {
        let num: Option<usize> = match num {
            KeypadButton::Zero => Some(0),
            KeypadButton::One => Some(1),
            KeypadButton::Two => Some(2),
            KeypadButton::Three => Some(3),
            KeypadButton::Four => Some(4),
            KeypadButton::Five => Some(5),
            KeypadButton::Six => Some(6),
            KeypadButton::Seven => Some(7),
            KeypadButton::Eight => Some(8),
            KeypadButton::Nine => Some(9),
            KeypadButton::Activate => None,
        };
        if let Some(n) = num {
            result += n * x;
            x *= 10;
        }
    }
    result
}

fn part1(inputs: Vec<KeypadButton>) -> Vec<Sequence> {
    let keypad_movements = precompute_keypad_movement();
    let first_bot = get_input_sequence(inputs, keypad_movements);
    chain_bots(first_bot, 2)
}

fn part2(inputs: Vec<KeypadButton>) -> Vec<Sequence> {
    let keypad_movements = precompute_keypad_movement();
    let first_bot = get_input_sequence(inputs, keypad_movements);
    // lol nope
    chain_bots(first_bot, 25)
}

fn chain_bots(mut inputs: Vec<Sequence>, count: usize) -> Vec<Sequence> {
    let directional_movements = precompute_directional_movement();
    for x in 0..count {
        println!("loop {x}");
        inputs = inputs
            .into_iter()
            .flat_map(|s| convert_input_sequence(s, &directional_movements))
            .min_set_by(|p1, p2| p1.len().cmp(&p2.len()));
    }
    inputs
}

fn precompute_keypad_movement() -> HashMap<(KeypadButton, KeypadButton), Vec<Vec<DirectionalButton>>>
{
    KEYPAD_BUTTONS
        .into_iter()
        .cartesian_product(KEYPAD_BUTTONS)
        .map(|(a, b)| {
            let (astar_paths, _) = astar_bag(
                &a,
                |button| keypad_neighbours(button),
                |_| 0,
                |cur| *cur == b,
            )
            .expect("some way must exist");
            let directions = astar_paths
                .into_iter()
                .map(|path| {
                    path.into_iter()
                        .tuple_windows()
                        .flat_map(|(a, b)| a.try_move_to(b))
                        .collect()
                })
                .collect();
            ((a, b), directions)
        })
        .collect()
}

fn precompute_directional_movement(
) -> HashMap<(DirectionalButton, DirectionalButton), Vec<Vec<DirectionalButton>>> {
    DIRECTIONAL_BUTTON
        .into_iter()
        .cartesian_product(DIRECTIONAL_BUTTON)
        .map(|(a, b)| {
            let (astar_paths, _) = astar_bag(
                &a,
                |button| directional_pad_neighbours(button),
                |_| 0,
                |cur| *cur == b,
            )
            .expect("some way must exist");
            let directions = astar_paths
                .into_iter()
                .map(|path| {
                    path.into_iter()
                        .tuple_windows()
                        .flat_map(|(a, b)| a.try_move_to(b))
                        .collect()
                })
                .collect();
            ((a, b), directions)
        })
        .collect()
}

fn directional_pad_neighbours(button: &DirectionalButton) -> Vec<(DirectionalButton, usize)> {
    use DirectionalButton::*;
    match button {
        Up => vec![Activate, Down],
        Down => vec![Left, Up, Right],
        Left => vec![Down],
        Right => vec![Down, Activate],
        Activate => vec![Up, Right],
    }
    .into_iter()
    .map(|b| (b, 1))
    .collect()
}

fn keypad_neighbours(button: &KeypadButton) -> Vec<(KeypadButton, usize)> {
    use KeypadButton::*;
    match button {
        Zero => vec![Activate, Two],
        One => vec![Four, Two],
        Two => vec![One, Five, Three, Zero],
        Three => vec![Two, Six, Activate],
        Four => vec![Seven, Five, One],
        Five => vec![Four, Eight, Six, Two],
        Six => vec![Five, Nine, Three],
        Seven => vec![Eight, Four],
        Eight => vec![Seven, Nine, Five],
        Nine => vec![Eight, Six],
        Activate => vec![Zero, Three],
    }
    .into_iter()
    .map(|b| (b, 1))
    .collect()
}

fn get_input_sequence(
    inputs: Vec<KeypadButton>,
    pre_compute: HashMap<(KeypadButton, KeypadButton), Vec<Vec<DirectionalButton>>>,
) -> Vec<Sequence> {
    let mut current = KeypadButton::Activate;
    let mut paths: Vec<Vec<DirectionalButton>> = vec![vec![]];
    for target in inputs {
        let possible_paths = pre_compute
            .get(&(current, target))
            .expect("there should always be a path");
        paths = paths
            .into_iter()
            .cartesian_product(possible_paths)
            .map(|(mut current_path, next_steps)| {
                let mut next_steps = next_steps.clone();
                current_path.append(&mut next_steps);
                current_path.push(DirectionalButton::Activate);
                current_path
            })
            .collect();
        current = target;
    }
    paths
        .into_iter()
        .min_set_by(|p1, p2| p1.len().cmp(&p2.len()))
        .into_iter()
        .map(|path| Sequence { inner: path })
        .collect()
}

fn convert_input_sequence(
    inputs: Sequence,
    pre_compute: &HashMap<(DirectionalButton, DirectionalButton), Vec<Vec<DirectionalButton>>>,
) -> Vec<Sequence> {
    let mut current = DirectionalButton::Activate;
    let mut paths: Vec<Vec<DirectionalButton>> = vec![vec![]];
    for target in inputs.inner {
        let possible_paths = pre_compute
            .get(&(current, target))
            .expect("there should always be a path");
        paths = paths
            .into_iter()
            .cartesian_product(possible_paths)
            .map(|(mut current_path, next_steps)| {
                let mut next_steps = next_steps.clone();
                current_path.append(&mut next_steps);
                current_path.push(DirectionalButton::Activate);
                current_path
            })
            .collect();
        current = target;
    }
    paths
        .into_iter()
        .min_set_by(|p1, p2| p1.len().cmp(&p2.len()))
        .into_iter()
        .map(|path| Sequence { inner: path })
        .collect()
}

fn parse(input: &str) -> Vec<KeypadButton> {
    input
        .chars()
        .map(|c| c.try_into().expect("valid aoc input"))
        .collect()
}

struct Sequence {
    inner: Vec<DirectionalButton>,
}

impl Sequence {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.inner {
            write!(f, "{}", b)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum DirectionalButton {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

const DIRECTIONAL_BUTTON: [DirectionalButton; 5] = [
    DirectionalButton::Up,
    DirectionalButton::Down,
    DirectionalButton::Right,
    DirectionalButton::Left,
    DirectionalButton::Activate,
];

impl DirectionalButton {
    fn try_move_to(&self, rhs: Self) -> Option<DirectionalButton> {
        match (self, rhs) {
            (DirectionalButton::Up, DirectionalButton::Down) => Some(DirectionalButton::Down),
            (DirectionalButton::Up, DirectionalButton::Activate) => Some(DirectionalButton::Right),
            (DirectionalButton::Down, DirectionalButton::Up) => Some(DirectionalButton::Up),
            (DirectionalButton::Down, DirectionalButton::Left) => Some(DirectionalButton::Left),
            (DirectionalButton::Down, DirectionalButton::Right) => Some(DirectionalButton::Right),
            (DirectionalButton::Left, DirectionalButton::Down) => Some(DirectionalButton::Right),
            (DirectionalButton::Right, DirectionalButton::Down) => Some(DirectionalButton::Left),
            (DirectionalButton::Right, DirectionalButton::Activate) => Some(DirectionalButton::Up),
            (DirectionalButton::Activate, DirectionalButton::Up) => Some(DirectionalButton::Left),
            (DirectionalButton::Activate, DirectionalButton::Right) => {
                Some(DirectionalButton::Down)
            }

            (_, _) => None,
        }
    }
}

impl Display for DirectionalButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DirectionalButton::Up => "^",
                DirectionalButton::Down => "v",
                DirectionalButton::Left => "<",
                DirectionalButton::Right => ">",
                DirectionalButton::Activate => "A",
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum KeypadButton {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

impl KeypadButton {
    fn try_move_to(&self, rhs: Self) -> Option<DirectionalButton> {
        match (self, rhs) {
            (KeypadButton::Zero, KeypadButton::Two) => Some(DirectionalButton::Up),
            (KeypadButton::Zero, KeypadButton::Activate) => Some(DirectionalButton::Right),
            (KeypadButton::One, KeypadButton::Two) => Some(DirectionalButton::Right),
            (KeypadButton::One, KeypadButton::Four) => Some(DirectionalButton::Up),
            (KeypadButton::Two, KeypadButton::Zero) => Some(DirectionalButton::Down),
            (KeypadButton::Two, KeypadButton::One) => Some(DirectionalButton::Left),
            (KeypadButton::Two, KeypadButton::Three) => Some(DirectionalButton::Right),
            (KeypadButton::Two, KeypadButton::Five) => Some(DirectionalButton::Up),
            (KeypadButton::Three, KeypadButton::Two) => Some(DirectionalButton::Left),
            (KeypadButton::Three, KeypadButton::Six) => Some(DirectionalButton::Up),
            (KeypadButton::Three, KeypadButton::Activate) => Some(DirectionalButton::Down),
            (KeypadButton::Four, KeypadButton::One) => Some(DirectionalButton::Down),
            (KeypadButton::Four, KeypadButton::Five) => Some(DirectionalButton::Right),
            (KeypadButton::Four, KeypadButton::Seven) => Some(DirectionalButton::Up),
            (KeypadButton::Five, KeypadButton::Two) => Some(DirectionalButton::Down),
            (KeypadButton::Five, KeypadButton::Four) => Some(DirectionalButton::Left),
            (KeypadButton::Five, KeypadButton::Six) => Some(DirectionalButton::Right),
            (KeypadButton::Five, KeypadButton::Eight) => Some(DirectionalButton::Up),
            (KeypadButton::Six, KeypadButton::Three) => Some(DirectionalButton::Down),
            (KeypadButton::Six, KeypadButton::Five) => Some(DirectionalButton::Left),
            (KeypadButton::Six, KeypadButton::Nine) => Some(DirectionalButton::Up),
            (KeypadButton::Seven, KeypadButton::Four) => Some(DirectionalButton::Down),
            (KeypadButton::Seven, KeypadButton::Eight) => Some(DirectionalButton::Right),
            (KeypadButton::Eight, KeypadButton::Five) => Some(DirectionalButton::Down),
            (KeypadButton::Eight, KeypadButton::Seven) => Some(DirectionalButton::Left),
            (KeypadButton::Eight, KeypadButton::Nine) => Some(DirectionalButton::Right),
            (KeypadButton::Nine, KeypadButton::Six) => Some(DirectionalButton::Down),
            (KeypadButton::Nine, KeypadButton::Eight) => Some(DirectionalButton::Left),
            (KeypadButton::Activate, KeypadButton::Zero) => Some(DirectionalButton::Left),
            (KeypadButton::Activate, KeypadButton::Three) => Some(DirectionalButton::Up),
            (_, _) => None,
        }
    }
}

const KEYPAD_BUTTONS: [KeypadButton; 11] = [
    KeypadButton::Zero,
    KeypadButton::One,
    KeypadButton::Two,
    KeypadButton::Three,
    KeypadButton::Four,
    KeypadButton::Five,
    KeypadButton::Six,
    KeypadButton::Seven,
    KeypadButton::Eight,
    KeypadButton::Nine,
    KeypadButton::Activate,
];

impl TryFrom<char> for KeypadButton {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'A' => Ok(Self::Activate),
            val => Err(format!("encountered {val}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "029A",
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
    )]
    #[case("980A", "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A")]
    #[case(
        "179A",
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    )]
    #[case(
        "456A",
        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"
    )]
    #[case(
        "379A",
        "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    )]
    fn test_complete(#[case] input: &str, #[case] result: &str) {
        let inputs = parse(input);
        assert!(part1(inputs)
            .iter()
            .map(|s| format!("{s}"))
            .any(|s| &s == result))
    }

    #[rstest]
    #[case("029A", vec![KeypadButton::Zero, KeypadButton::Two, KeypadButton::Nine, KeypadButton::Activate])]
    #[case("0123456789A", vec![KeypadButton::Zero, KeypadButton::One, KeypadButton::Two, KeypadButton::Three, KeypadButton::Four, KeypadButton::Five, KeypadButton::Six, KeypadButton::Seven,KeypadButton::Eight, KeypadButton::Nine, KeypadButton::Activate])]
    fn test_parse(#[case] input: &str, #[case] expected: Vec<KeypadButton>) {
        assert_eq!(parse(input), expected)
    }

    #[test]
    fn test_029A() {
        let inputs = parse("029A");
        let pre_compute = precompute_keypad_movement();
        let mut optional_paths: Vec<String> = get_input_sequence(inputs, pre_compute)
            .into_iter()
            .map(|p| format!("{p}"))
            .collect();
        optional_paths.sort();
        assert_eq!(
            optional_paths,
            vec!["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"]
        )
    }
}
