use aoc::{get_cardinal_neighbours, Grid, Point};
use pathfinding::prelude::*;

const WIDTH: usize = 71;
const HIGHT: usize = 71;
const GOAL: Point = Point(70, 70);


fn main() {
    let input = include_str!("../input.txt");
    let points = parse(input);
    let mut grid: Grid<Cell> = Grid::new(HIGHT, WIDTH);
    for point in points.iter().take(1024) {
        grid[point] = Cell::Corrupted;
    }
    let x = astar(
        &Point(0, 0),
        |p: &Point| successors(p, &grid),
        |p| manhattan(p, &GOAL),
        |p| p == &GOAL,
    )
    .expect("a solution exists");
    println!("part1: {}", x.1);

    for byte in points.iter().skip(1024) {
        grid[byte] = Cell::Corrupted;
        let r = astar(
            &Point(0, 0),
            |p: &Point| successors(p, &grid),
            |p| manhattan(p, &GOAL),
            |p| p == &GOAL,
        );
        if r.is_none() {
            println!("part2: {},{}", byte.1, byte.0);
            break;
        }
    }
}

fn manhattan(a: &Point, b: &Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn successors(p: &Point, grid: &Grid<Cell>) -> Vec<(Point, usize)>{
    get_cardinal_neighbours(&grid, p)
    .into_iter()
    .filter(|p| grid[p] != Cell::Corrupted)
    .map(|p| (p, 1))
    .collect()
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Cell {
    #[default]
    Empty,
    Corrupted,
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .split(",")
                .take(2)
                .map(|x| x.parse::<usize>().expect("valid input"))
                .collect();
            assert!(nums.len() == 2);
            Point(nums[1], nums[0])
        })
        .collect()
}
