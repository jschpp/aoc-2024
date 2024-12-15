use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

use aoc::{Grid, Point};
mod parse;
mod types;
use parse::*;
use types::*;

fn main() {
    let input = include_str!("../input.txt");
    let (_input, (mut grid, path)) = parse(input).unwrap();
    // dbg!(&input, &grid, &path);
    let mut bigger_grid = blow_up_grid(&grid);
    move_robot(&mut grid, &path);
    print_grid(&grid);
    let part1 = part1(&grid);
    println!("part1: {part1}");

    move_robot_p2(&mut bigger_grid, &path);
    print_grid(&bigger_grid);
    let part2 = part2(&bigger_grid);
    println!("part1: {part2}");
}

fn move_robot(grid: &mut Grid<Cell>, path: &[Direction]) {
    let mut curr_pos: Point = grid
        .indexed_iter()
        .find(|(_, c)| **c == Cell::Robot)
        .map(|(pos, _)| pos.into())
        .expect("Robot exists");
    // print_grid(&grid);
    for dir in path {
        // println!("Move {:?}:", dir);
        let dir_vec: (i32, i32) = (*dir).into();
        if let Some(new_pos) = curr_pos + dir_vec {
            match grid[new_pos] {
                Cell::Wall => {}
                Cell::Crate => {
                    if let Some(new_pos) = move_crates_p1(grid, new_pos, dir_vec) {
                        curr_pos = move_and_swap(grid, curr_pos, new_pos);
                    }
                }
                Cell::Empty => {
                    curr_pos = move_and_swap(grid, curr_pos, new_pos);
                }
                Cell::Robot => unreachable!(),
            }
        }
        // print_grid(&grid);
    }
}

fn move_crates_p1(grid: &mut Grid<Cell>, pos: Point, dir: (i32, i32)) -> Option<Point> {
    if let Some(new_position) = pos + dir {
        match grid[new_position] {
            Cell::Wall => None,
            Cell::Crate => {
                if let Some(new_pos) = move_crates_p1(grid, new_position, dir) {
                    move_and_swap(grid, pos, new_pos);
                    Some(pos)
                } else {
                    None
                }
            }
            Cell::Robot => unreachable!(),
            Cell::Empty => {
                move_and_swap(grid, pos, new_position);
                Some(pos)
            }
        }
    } else {
        None
    }
}

fn move_and_swap(grid: &mut Grid<Cell>, current: Point, new: Point) -> Point {
    grid.swap(new.into(), current.into());
    new
}

fn part1(grid: &Grid<Cell>) -> usize {
    grid.indexed_iter()
        .flat_map(|((line, col), c)| {
            if *c == Cell::Crate {
                Some(100 * line + col)
            } else {
                None
            }
        })
        .sum()
}

fn blow_up_grid(grid: &Grid<Cell>) -> Grid<BiggerCell> {
    let mut bigger_grid: Grid<BiggerCell> = Grid::new(grid.rows(), grid.cols() * 2);
    for (pos, cell) in grid.indexed_iter() {
        let pos: Point = (pos.0, pos.1 * 2).into();
        let right_pos = (pos + (0, 1)).expect("valid");
        match cell {
            Cell::Wall => {
                bigger_grid[pos] = BiggerCell::Wall;
                bigger_grid[right_pos] = BiggerCell::Wall;
            }
            Cell::Crate => {
                bigger_grid[pos] = BiggerCell::CrateLeft;
                bigger_grid[right_pos] = BiggerCell::CrateRight;
            }
            Cell::Robot => bigger_grid[pos] = BiggerCell::Robot,
            Cell::Empty => continue,
        }
    }
    bigger_grid
}

fn move_crates_p2(grid: &mut Grid<BiggerCell>, pos: Point, dir: (i32, i32)) -> Option<Point> {
    let grid_copy = grid.clone();
    let mut seen: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(pos);
    seen.insert(pos);
    let mut encountered_wall = false;

    while !queue.is_empty() && !encountered_wall {
        let pos = queue.pop_front().unwrap();
        let next_pos =
            (pos + dir).expect("should be valid since there is a border around the field");
        if seen.contains(&next_pos) {
            continue;
        }
        let next_pos_right =
            (next_pos + (0, 1)).expect("should be valid since there is a border around the field");
        let next_pos_left =
            (next_pos + (0, -1)).expect("should be valid since there is a border around the field");
        match grid[next_pos] {
            BiggerCell::Wall => {
                encountered_wall = true;
            }
            BiggerCell::CrateLeft => {
                queue.push_back(next_pos);
                queue.push_back(next_pos_right);
                seen.insert(next_pos);
                seen.insert(next_pos_right);
            }
            BiggerCell::CrateRight => {
                queue.push_back(next_pos);
                queue.push_back(next_pos_left);
                seen.insert(next_pos);
                seen.insert(next_pos_left);
            }
            BiggerCell::Empty => {}
            BiggerCell::Robot => unreachable!(),
        }
    }

    let mut new_robot_position = None;
    if !encountered_wall {
        for point in seen.iter() {
            grid[point] = BiggerCell::Empty;
        }
        for point in seen.iter() {
            let new_pos = (point + dir).expect("valid");
            grid[new_pos] = grid_copy[point];
            if grid[new_pos] == BiggerCell::Robot {
                new_robot_position = Some(new_pos);
            }
        }
    }
    new_robot_position
}

fn move_robot_p2(grid: &mut Grid<BiggerCell>, path: &[Direction]) {
    let mut curr_pos: Point = grid
        .indexed_iter()
        .find(|(_, c)| **c == BiggerCell::Robot)
        .map(|(pos, _)| pos.into())
        .expect("Robot exists");
    for dir in path {
        if let Some(pos) = move_crates_p2(grid, curr_pos, (*dir).into()) {
            curr_pos = pos
        }
    }
}

fn part2(grid: &Grid<BiggerCell>) -> usize {
    grid.indexed_iter()
        .flat_map(|((line, col), c)| {
            if *c == BiggerCell::CrateLeft {
                Some(100 * line + col)
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
fn print_grid<T>(grid: &Grid<T>)
where
    T: Debug,
{
    for line in grid.iter_rows() {
        for cell in line.into_iter() {
            print!("{:?}", cell)
        }
        println!()
    }
    println!()
}
