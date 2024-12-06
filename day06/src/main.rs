use rayon::prelude::*;
use std::{
    collections::HashSet,
    fmt::Display,
    iter::Sum,
    ops::{Add, Index, IndexMut},
};

#[derive(Debug, Default, Clone)]
struct Grid {
    data: Vec<Vec<Place>>,
    start: Position,
    found_loop: bool,
}

impl Grid {
    fn walk_guard(&mut self) {
        let mut direction: Position = Position(-1, 0);
        let mut current_position = self.start;
        let mut path: HashSet<(Position, Position)> = HashSet::new();
        'outer: loop {
            self[current_position] = Place::Visited;
            let cache = (current_position, direction);
            if path.contains(&cache) {
                // println!("found loop");
                self.found_loop = true;
                break;
            }
            path.insert(cache);
            'inner: loop {
                let next_option = current_position + direction;
                if next_option.0 < 0
                    || next_option.0 >= self.data.len() as isize
                    || next_option.1 < 0
                    || next_option.1 >= self.data[0].len() as isize
                {
                    break 'outer;
                }
                match &self[next_option] {
                    Place::Obstacle => direction = direction.rotate_90(),
                    _ => {
                        current_position = next_option;
                        break 'inner;
                    }
                }
            }
        }
    }

    fn count(&self) -> PlaceCount {
        self.data
            .iter()
            .flat_map(|line| {
                line.iter().map(|place| match place {
                    Place::Empty => PlaceCount {
                        empty: 1,
                        visited: 0,
                        obstacles: 0,
                    },
                    Place::Visited => PlaceCount {
                        empty: 0,
                        visited: 1,
                        obstacles: 0,
                    },
                    Place::Obstacle => PlaceCount {
                        empty: 0,
                        visited: 0,
                        obstacles: 1,
                    },
                })
            })
            .sum()
    }

    fn place_obstacle(&self, pos: Position) -> Self {
        let mut out = self.clone();
        out[pos] = Place::Obstacle;
        out
    }
}

#[derive(Debug, Default)]
struct PlaceCount {
    empty: usize,
    visited: usize,
    obstacles: usize,
}

impl Add<PlaceCount> for PlaceCount {
    type Output = Self;

    fn add(self, rhs: PlaceCount) -> Self::Output {
        PlaceCount {
            empty: self.empty + rhs.empty,
            visited: self.visited + rhs.visited,
            obstacles: self.obstacles + rhs.obstacles,
        }
    }
}

impl Sum for PlaceCount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(PlaceCount::default(), |acc, place| acc + place)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.data {
            for place in line {
                write!(f, "{}", place)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl Position {
    fn rotate_90(&self) -> Self {
        // (x, y) rotated 90 degrees clockwise around (0, 0) is (y, -x)
        Self(self.1, -self.0)
    }
}

impl Add<Position> for Position {
    type Output = Position;

    // Required method
    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    Empty,
    Visited,
    Obstacle,
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Place::Empty => write!(f, " "),
            Place::Visited => write!(f, "X"),
            Place::Obstacle => write!(f, "#"),
        }
    }
}

impl Default for Place {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<char> for Place {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Obstacle,
            _ => Self::Empty,
        }
    }
}

impl Index<Position> for Grid {
    type Output = Place;

    fn index(&self, index: Position) -> &Self::Output {
        &self.data[index.0 as usize][index.1 as usize]
    }
}

impl IndexMut<Position> for Grid {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.data[index.0 as usize][index.1 as usize]
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut g: Vec<Vec<Place>> = Vec::default();
    let mut start: Position = Position::default();
    for (line_idx, line) in input.lines().enumerate() {
        g.push(Vec::default());
        for (char_idx, c) in line.chars().enumerate() {
            if c == '^' {
                start = Position(line_idx as isize, char_idx as isize);
            }
            g[line_idx].push(c.into());
        }
    }
    let grid = Grid {
        data: g,
        start,
        found_loop: false,
    };
    let mut initial_grid = grid.clone();
    initial_grid.walk_guard();
    println!("part1: {}", initial_grid.count().visited);

    let loop_count: usize = initial_grid
        .data
        .iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            let line_idx = line_idx;
            line.iter()
                .enumerate()
                .filter(|(_, p)| **p == Place::Visited)
                .map(move |(pos, _)| Position(line_idx as isize, pos as isize))
        })
        .collect::<Vec<_>>()
        .par_iter()
        .flat_map(|pos| {
            let mut grid = grid.clone().place_obstacle(*pos);
            grid.walk_guard();
            if grid.found_loop {
                Some(())
            } else {
                None
            }
        })
        .count();

    println!("part2: {loop_count}");
}
