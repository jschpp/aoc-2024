use std::{
    collections::HashSet,
    ops::{self, Add},
    sync::{Arc, RwLock},
};
#[macro_use]
extern crate impl_ops;

use grid::Grid;

fn main() {
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect();
    let grid = grid.into();
    let result = solve(grid);
    // dbg!(get_valid_neighbours(&grid, &Point(0, 0)));
    println!("{result}");
}

fn solve(grid: Grid<usize>) -> usize {
    let starting_pos: Vec<Point> = grid
        .indexed_iter()
        .filter(|((_, _), x)| **x == 0)
        .map(|((line_idx, col_idx), _)| Point(line_idx, col_idx))
        .collect();
    starting_pos
        .into_iter()
        .map(|p| {
            let reached_nine: Arc<RwLock<HashSet<Point>>> =
                Arc::new(RwLock::new(HashSet::default()));
            let chain = vec![p];
            travel(chain, &grid, reached_nine.clone())
        })
        .sum()
}

fn travel(
    chain: Vec<Point>,
    grid: &Grid<usize>,
    reached_nine: Arc<RwLock<HashSet<Point>>>,
) -> usize {
    // check is chain is complete
    if chain.iter().map(|p| grid[p.into()]).collect::<Vec<usize>>()
        == vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    {
        // Enable this for part1
        // reached_nine.write().unwrap().insert(chain[chain.len() - 1]);
        return 1;
    }
    let next = &chain[chain.len() - 1];
    let n: Vec<Point> = get_valid_neighbours(grid, next)
        .into_iter()
        .filter(|p| {
            grid[(*p).into()] - grid[next.into()] == 1 && !reached_nine.read().unwrap().contains(p)
        })
        .collect();
    if n.is_empty() {
        return 0;
    }
    n.iter()
        .map(|next| {
            let mut tmp_chain = chain.clone();
            tmp_chain.push(*next);
            travel(tmp_chain, grid, reached_nine.clone())
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}

impl From<&Point> for (usize, usize) {
    fn from(value: &Point) -> Self {
        (value.0, value.1)
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        if rhs.0 >= 0 && rhs.1 >= 0 {
            Some(self + Point(rhs.0 as usize, rhs.1 as usize))
        } else {
            let l = self.0 as i32 + rhs.0;
            let c = self.1 as i32 + rhs.1;
            if l >= 0 && c >= 0 {
                Some(Self(l as usize, c as usize))
            } else {
                None
            }
        }
    }
}

impl Add<(i32, i32)> for &Point {
    type Output = Option<Point>;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        if rhs.0 >= 0 && rhs.1 >= 0 {
            Some(self + Point(rhs.0 as usize, rhs.1 as usize))
        } else {
            let l = self.0 as i32 + rhs.0;
            let c = self.1 as i32 + rhs.1;
            if l >= 0 && c >= 0 {
                Some(Point(l as usize, c as usize))
            } else {
                None
            }
        }
    }
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point(a.0 + b.0, a.1 + b.1)});

fn get_valid_neighbours(grid: &Grid<usize>, p: &Point) -> Vec<Point> {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    DIRECTIONS
        .iter()
        .flat_map(|dir| p + *dir)
        .filter(|p| grid.rows() > p.0 && grid.cols() > p.1)
        .collect()
}
