use aoc::{get_cardinal_neighbours, Grid, Point};
use pathfinding::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::Display;

const MIN_DIFF: usize = 100;

fn main() {
    let input = include_str!("../input.txt");
    let g = parse(input);

    let part1_result = part1(g);
    println!("part1: {part1_result}");
}

#[derive(Debug, Clone)]
struct RaceTrack {
    grid: Grid<Cell>,
    start: Point,
    end: Point,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    #[default]
    Empty,
    Wall,
}

fn part1(track: RaceTrack) -> usize {
    let original = search_path(&track);
    let pairs: Vec<(Point, Point)> = track
        .grid
        .indexed_iter()
        .filter(|(p, c)| {
            **c == Cell::Wall
                && get_cardinal_neighbours(&track.grid, &(*p).into())
                    .iter()
                    .any(|coord| track.grid[coord] == Cell::Empty)
        })
        .flat_map(|(p, _)| {
            get_cardinal_neighbours(&track.grid, &p.into())
                .into_iter()
                .map(move |coord| (p.into(), coord))
        })
        .filter(|(_, b)| track.grid[b] == Cell::Empty)
        .collect();
    let grid_set = pairs
        .into_iter()
        .map(|(a, _)| {
            let mut tmp = track.grid.clone();
            tmp[a] = Cell::Empty;
            tmp
        })
        .collect::<HashSet<_>>();
    grid_set
        .into_par_iter()
        .map(|grid| {
            let t = RaceTrack {
                grid,
                start: track.start,
                end: track.end,
            };
            search_path(&t)
        })
        .filter(|length| *length <= original - MIN_DIFF)
        .count()
}

fn search_path(track: &RaceTrack) -> usize {
    let search_result = astar(
        &track.start,
        |p| neighbours(p, &track.grid),
        |p| p.0.abs_diff(track.end.0) + p.1.abs_diff(track.end.1),
        |p| *p == track.end,
    )
    .expect("there should be a path");
    search_result.1
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => ' ',
                Cell::Wall => '#',
            }
        )
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<Cell>) {
    for row in grid.iter_rows() {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

fn neighbours(p: &Point, grid: &Grid<Cell>) -> Vec<(Point, usize)> {
    get_cardinal_neighbours(grid, p)
        .into_iter()
        .filter(|coord| grid[coord] != Cell::Wall)
        .map(|coord| (coord, 1))
        .collect()
}

fn parse(input: &str) -> RaceTrack {
    let mut start: Point = (0, 0).into();
    let mut end: Point = (0, 0).into();
    let rows = input.lines().count();
    let cols = input
        .lines()
        .next()
        .expect("there should be at least one line")
        .len();
    let mut grid: Grid<Cell> = Grid::new(rows, cols);
    for (row_idx, row) in input.lines().enumerate() {
        for (c_idx, c) in row.chars().enumerate() {
            let p = (row_idx, c_idx);
            match c {
                '#' => grid[p] = Cell::Wall,
                'S' => start = p.into(),
                'E' => end = p.into(),
                _ => {}
            }
        }
    }
    RaceTrack { grid, start, end }
}
