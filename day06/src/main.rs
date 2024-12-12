use aoc::{Grid, Point};
use itertools::Itertools;
use nom::character::is_newline;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Direction(i32, i32);

impl Direction {
    fn rotate_90(&self) -> Self {
        // (x, y) rotated 90 degrees clockwise around (0, 0) is (y, -x)
        Self(self.1, -self.0)
    }

    fn i32_tuple(&self) -> (i32, i32) {
        self.into()
    }
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        (value.0, value.1)
    }
}

impl From<&Direction> for (i32, i32) {
    fn from(value: &Direction) -> Self {
        (value.0, value.1)
    }
}

struct Path {
    data: Vec<Point>,
    found_loop: bool,
}

fn walk_guard(start: Point, grid: &Grid<Place>) -> Path {
    let mut direction: Direction = Direction(-1, 0);
    let mut current_position = start;
    let mut path: HashSet<(Point, Direction)> = HashSet::new();
    let mut found_loop = false;
    'outer: loop {
        let cache = (current_position, direction);
        if path.contains(&cache) {
            found_loop = true;
            break;
        }
        path.insert(cache);
        'inner: loop {
            let next_option: Option<Point> = current_position + direction.i32_tuple();
            if let Some(next) = next_option {
                if next.0 >= grid.rows() || next.1 >= grid.cols() {
                    break 'outer;
                }
                match grid[next] {
                    Place::Obstacle => direction = direction.rotate_90(),
                    _ => {
                        current_position = next;
                        break 'inner;
                    }
                }
            } else {
                break 'outer;
            }
        }
    }
    let path = path.into_iter().map(|(point, _)| point).unique().collect();
    Path {
        data: path,
        found_loop,
    }
}

fn place_obstacle(grid: &Grid<Place>, pos: Point) -> Grid<Place> {
    let mut out = grid.clone();
    out[pos] = Place::Obstacle;
    out
}

#[allow(unused)]
fn visit_all(grid: &mut Grid<Place>, path: &[Point]) {
    for point in path {
        grid[point] = Place::Visited
    }
}

#[allow(unused)]
fn print_grid(grid: &Grid<Place>) {
    for line in grid.iter_rows() {
        for place in line {
            print!("{}", place);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    Empty,
    Visited,
    Obstacle,
    Start,
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Place::Empty => write!(f, "."),
            Place::Visited => write!(f, "X"),
            Place::Obstacle => write!(f, "#"),
            Place::Start => write!(f, "^"),
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
            '^' => Self::Start,
            _ => Self::Empty,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let (start, g) = parse_map(input);
    let path = walk_guard(start, &g);

    println!("part1: {}", path.data.len());

    let loop_count: usize = path.data[1..path.data.len()]
        .into_par_iter()
        .flat_map(|point| {
            let new_grid = place_obstacle(&g, *point);
            let path = walk_guard(start, &new_grid);
            if path.found_loop {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("part2: {loop_count}");
}

fn parse_map(input: &str) -> (Point, Grid<Place>) {
    let width = input
        .lines()
        .next()
        .expect("there should be at least one row")
        .chars()
        .count();
    let data = input
        .chars()
        .filter(|c| !is_newline(*c as u8))
        .map(|c| c.into())
        .collect::<Vec<Place>>();

    let mut grid = Grid::from_vec(data, width);

    let ((line, col), _) = grid
        .indexed_iter()
        .find(|(_, place)| **place == Place::Start)
        .expect("start should exist");
    let start = Point(line, col);
    grid[start] = Place::Empty;

    (start, grid)
}
