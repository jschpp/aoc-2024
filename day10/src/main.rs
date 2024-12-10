use std::{
    collections::HashSet,
    ops::{self, Add, Index},
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

    println!("part1 {}", part1(&grid));
    println!("part2 {}", part2(&grid));
}

fn part1(grid: &Grid<usize>) -> usize {
    let chains = solve(grid);
    chains
        .iter()
        .map(|chain| (chain[0], chain[chain.len() - 1]))
        .collect::<HashSet<(Point, Point)>>()
        .len()
}

fn part2(grid: &Grid<usize>) -> usize {
    let chains = solve(grid);
    chains.len()
}

fn solve(grid: &Grid<usize>) -> Vec<Vec<Point>> {
    let starting_pos: Vec<Point> = grid
        .indexed_iter()
        .filter(|((_, _), x)| **x == 0)
        .map(|((line_idx, col_idx), _)| Point(line_idx, col_idx))
        .collect();
    starting_pos
        .into_iter()
        .flat_map(|p| {
            let chain = vec![p];
            travel(chain, grid)
        })
        .collect()
}

fn travel(chain: Vec<Point>, grid: &Grid<usize>) -> Vec<Vec<Point>> {
    // check is chain is complete
    // since we only add valid (bigger by one number) candidates to a chain the chain must me correct after hitting 10 nodes
    if chain.len() == 10 {
        return vec![chain];
    }
    let next = &chain[chain.len() - 1];
    let n: Vec<Point> = get_valid_neighbours(grid, next)
        .into_iter()
        .filter(|p| grid[p] - grid[next] == 1)
        .collect();
    if n.is_empty() {
        return Vec::default();
    }
    n.iter()
        .flat_map(|next| {
            let mut tmp_chain = chain.clone();
            tmp_chain.push(*next);
            travel(tmp_chain, grid)
                .into_iter()
                .filter(|x| x.len() == 10)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self[(index.0, index.1)]
    }
}

impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Point) -> &Self::Output {
        &self[*index]
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

/// get the valid cardinal neighbour for a given position in the grid
///
/// checks wether the neighbours are in the grid or not
/// Will return an empty vector if there are no neighbours
fn get_valid_neighbours(grid: &Grid<usize>, p: &Point) -> Vec<Point> {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    DIRECTIONS
        .iter()
        .flat_map(|dir| p + *dir)
        .filter(|p| grid.rows() > p.0 && grid.cols() > p.1)
        .collect()
}
