use aoc::{Grid, Point};
mod parse;
mod types;
use parse::*;
use types::*;

fn main() {
    let input = include_str!("../input.txt");
    let (_input, (mut grid, path)) = parse(input).unwrap();
    // dbg!(&input, &grid, &path);
    move_robot(&mut grid, path);
    print_grid(&grid);
    let part1 = part1(&grid);
    println!("part1: {part1}");
}

fn move_robot(grid: &mut Grid<Cell>, path: Vec<Direction>) {
    let mut curr_pos: Point = grid
        .indexed_iter()
        .find(|(_, c)| **c == Cell::Robot)
        .map(|(pos, _)| pos.into())
        .expect("Robot exists");
    // print_grid(&grid);
    for dir in path {
        // println!("Move {:?}:", dir);
        let dir_vec: (i32, i32) = dir.into();
        if let Some(new_pos) = curr_pos + dir_vec {
            match grid[new_pos] {
                Cell::Wall => {}
                Cell::Crate => {
                    if let Some(new_pos) = move_crates(grid, new_pos, dir_vec) {
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

fn move_crates(grid: &mut Grid<Cell>, pos: Point, dir: (i32, i32)) -> Option<Point> {
    if let Some(new_position) = pos + dir {
        match grid[new_position] {
            Cell::Wall => None,
            Cell::Crate => {
                if let Some(new_pos) = move_crates(grid, new_position, dir) {
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

#[allow(dead_code)]
fn print_grid(grid: &Grid<Cell>) {
    for line in grid.iter_rows() {
        for cell in line.into_iter() {
            print!("{:?}", cell)
        }
        println!()
    }
    println!()
}
