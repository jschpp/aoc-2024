use super::types::*;
use std::collections::HashMap;

pub fn run() {
    let content = include_str!("../input.txt");

    let parsed: Vec<Letter> = content
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.chars().enumerate().flat_map(move |(c_idx, c)| {
                Some(Letter {
                    letter: c.into(),
                    position: Position {
                        line: line_idx.try_into().expect("valid"),
                        letter: c_idx.try_into().expect("valid"),
                    },
                })
            })
        })
        .collect();
    let max_idx = parsed[parsed.len() - 1].position;

    let mut letter_map: HashMap<Position, LetterOption> =
        HashMap::with_capacity(max_idx.letter as usize * max_idx.line as usize);
    for letter in parsed.iter() {
        letter_map.insert(letter.position, letter.letter);
    }

    let count: usize = parsed
        .iter()
        .filter(|l| l.letter == LetterOption::X)
        .flat_map(|l| {
            get_word_options(l.position, max_idx)
                .into_iter()
                .map(|word| word.iter().map(|p| letter_map[p]).collect::<Vec<_>>())
                .filter(|option| is_valid(option))
                .collect::<Vec<_>>()
        })
        .count();
    println!("{count}")
}

fn get_word_options(pos: Position, max_grid: Position) -> Vec<Vec<Position>> {
    use Position as p;
    let mut options: Vec<Vec<Position>> = Vec::new();
    // | X | M | A | S |
    // |   |   |   |   |
    // |   |   |   |   |
    // |   |   |   |   |
    options.push(
        [p::new(0, 0), p::new(0, 1), p::new(0, 2), p::new(0, 3)]
            .iter()
            .map(|p| pos + *p)
            .collect(),
    );

    // | S | A | M | X |
    // |   |   |   |   |
    // |   |   |   |   |
    // |   |   |   |   |
    options.push(
        [p::new(0, 0), p::new(0, -1), p::new(0, -2), p::new(0, -3)]
            .iter()
            .map(|p| pos + *p)
            .collect(),
    );

    // | X |   |   |   |
    // | M |   |   |   |
    // | A |   |   |   |
    // | S |   |   |   |
    options.push(
        [p::new(0, 0), p::new(1, 0), p::new(2, 0), p::new(3, 0)]
            .iter()
            .map(|p| pos + *p)
            .collect(),
    );

    // | S |   |   |   |
    // | A |   |   |   |
    // | M |   |   |   |
    // | X |   |   |   |
    options.push(
        [p::new(0, 0), p::new(-1, 0), p::new(-2, 0), p::new(-3, 0)]
            .iter()
            .map(|p| pos + *p)
            .collect(),
    );

    // | S |   |   |   |
    // |   | A |   |   |
    // |   |   | M |   |
    // |   |   |   | X |
    options.push(
        [p::new(0, 0), p::new(-1, -1), p::new(-2, -2), p::new(-3, -3)]
            .iter()
            .map(|p| pos + *p)
            .collect(),
    );

    // |   |   |   | S |
    // |   |   | A |   |
    // |   | M |   |   |
    // | X |   |   |   |
    options.push(
        [p::new(0, 0), p::new(-1, 1), p::new(-2, 2), p::new(-3, 3)]
            .iter()
            .map(|p| pos + *p)
            .collect(),
    );

    // | X |   |   |   |
    // |   | M |   |   |
    // |   |   | A |   |
    // |   |   |   | S |
    options.push(
        [p::new(0, 0), p::new(1, 1), p::new(2, 2), p::new(3, 3)]
            .iter()
            .map(|p| pos + *p)
            .collect(),
    );

    // |   |   |   | X |
    // |   |   | M |   |
    // |   | A |   |   |
    // | S |   |   |   |
    options.push(
        [p::new(0, 0), p::new(1, -1), p::new(2, -2), p::new(3, -3)]
            .iter()
            .map(|p| pos + *p)
            .collect(),
    );

    options
        .into_iter()
        .filter(|option| {
            option.iter().all(|p| {
                p.letter >= 0
                    && p.letter <= max_grid.letter
                    && p.line >= 0
                    && p.line <= max_grid.line
            })
        })
        .collect()
}

fn is_valid(word: &[LetterOption]) -> bool {
    // let word: Vec<LetterOption> = chain.iter().map(|l| l.letter).collect();
    word == vec![
        LetterOption::X,
        LetterOption::M,
        LetterOption::A,
        LetterOption::S,
    ]
}
