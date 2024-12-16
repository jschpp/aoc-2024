use aoc::{get_cardinal_neighbours, Grid, Point};
use glam::IVec2;
use pathfinding::directed::{astar, dijkstra};
use std::{collections::HashSet, fmt::Debug};

fn main() {
    let input = include_str!("../input.txt");
    let maze = parse(input);
    let (path, p1_cost) = dijkstra::dijkstra(
        &(maze.start, IVec2::X),
        |(p, dir)| successors((*p, *dir), &maze.grid),
        |(p, dir)| *p == maze.end,
    )
    .unwrap();
    println!("part1 {p1_cost}");

    // part2
    let (astar_paths, p2_cost) = astar::astar_bag(
        &(maze.start, IVec2::X),
        |(p, dir)| successors((*p, *dir), &maze.grid),
        |_| path.len() + 1,
        |(p, dir)| *p == maze.end,
    )
    .unwrap();
    assert_eq!(p1_cost, p2_cost);
    let mut seen: HashSet<Point> = HashSet::new();
    for path in astar_paths {
        path.into_iter().for_each(|(p, _dir)| {
            seen.insert(p);
        })
    }
    println!("part1 {}", seen.len())
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
    let vecs = input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(col_idx, col)| match col {
                    '.' => Some(Cell::Empty),
                    '#' => Some(Cell::Wall),
                    'S' => {
                        start = Point(line_idx, col_idx);
                        Some(Cell::Empty)
                    }
                    'E' => {
                        end = Point(line_idx, col_idx);
                        Some(Cell::Empty)
                    }
                    val => panic!("encountered {val}"),
                })
                .collect()
        })
        .collect::<Vec<Vec<Cell>>>();
    let cols = vecs[0].len();
    Maze {
        start,
        end,
        grid: Grid::from_vec(vecs.into_iter().flatten().collect(), cols),
    }
}
