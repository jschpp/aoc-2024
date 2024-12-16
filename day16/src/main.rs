use aoc::{get_cardinal_neighbours, Grid, Point};
use glam::IVec2;
use pathfinding::directed::{astar, dijkstra};
use std::{collections::HashSet, fmt::Debug};

fn main() {
    let input = include_str!("../input.txt");
    let maze = parse(input);

    // Part1 use dijkstra to find shortest path
    // every step will yield a Point as well as an IVec2 describing the direction
    // the reindeer is currently facing
    let (_path, p1_cost) = dijkstra::dijkstra(
        &(maze.start, IVec2::X),
        |(p, dir)| successors((*p, *dir), &maze.grid),
        |(p, _)| *p == maze.end,
    )
    .unwrap();
    println!("part1 {p1_cost}");

    // Part2
    // Using A* to find _all_ shortest paths in the maze.
    let (astar_paths, p2_cost) = astar::astar_bag(
        &(maze.start, IVec2::X),
        |(p, dir)| successors((*p, *dir), &maze.grid),
        |(p, _)| heuristic(p, &maze.end),
        |(p, _)| *p == maze.end,
    )
    .unwrap();

    assert_eq!(p1_cost, p2_cost);

    // Check wich nodes were along at least one shortest path
    let mut seen: HashSet<Point> = HashSet::new();
    for path in astar_paths {
        path.into_iter().for_each(|(p, _)| {
            seen.insert(p);
        })
    }
    println!("part1 {}", seen.len())
}

fn heuristic(a: &Point, goal: &Point) -> usize {
    a.0.abs_diff(goal.0) + a.1.abs_diff(goal.1)
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    #[default]
    Empty,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => ".",
                Self::Wall => "#",
            }
        )
    }
}

#[derive(Debug)]
struct Maze {
    start: Point,
    end: Point,
    grid: Grid<Cell>,
}

fn successors((current, dir): (Point, IVec2), grid: &Grid<Cell>) -> Vec<((Point, IVec2), usize)> {
    get_cardinal_neighbours(grid, &current)
        .into_iter()
        .filter(|p| grid[*p] != Cell::Wall)
        .map(|p| {
            if Point(
                (current.0 as i32 + dir.y) as usize,
                (current.1 as i32 + dir.x) as usize,
            ) == p
            {
                ((p, dir), 1)
            } else {
                let new_dir = IVec2 {
                    x: p.1 as i32 - current.1 as i32,
                    y: p.0 as i32 - current.0 as i32,
                };
                ((p, new_dir), 1001)
            }
        })
        .collect()
}

fn parse(input: &str) -> Maze {
    let mut start: Point = Point(0, 0);
    let mut end: Point = Point(0, 0);
    let mut vecs: Vec<Cell> = Vec::with_capacity(input.len());
    let mut cols = 0;
    for (line_idx, line) in input.lines().enumerate() {
        for (col_idx, col) in line.chars().enumerate() {
            if col == '#' {
                vecs.push(Cell::Wall)
            } else {
                vecs.push(Cell::Empty);
                if col == 'S' {
                    start = Point(line_idx, col_idx);
                }
                if col == 'E' {
                    end = Point(line_idx, col_idx);
                }
            }
            cols = cols.max(col_idx + 1);
        }
    }

    Maze {
        start,
        end,
        grid: Grid::from_vec(vecs, cols),
    }
}
