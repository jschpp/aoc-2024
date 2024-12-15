use crate::types::*;
use aoc::Grid;
use nom::{
    character::{
        complete::{anychar, line_ending},
        streaming::one_of,
    },
    combinator::all_consuming,
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    one_of("@#.O").map(|c| c.into()).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(parse_cell)(input)
}

fn parse_grid(input: &str) -> IResult<&str, Grid<Cell>> {
    let (input, cells) = separated_list1(line_ending, parse_line)(input)?;
    let cols = cells[0].len();
    let flattened_cells = cells.into_iter().flatten().collect();
    let grid: Grid<Cell> = Grid::from_vec(flattened_cells, cols);
    Ok((input, grid))
}

pub fn parse(input: &str) -> IResult<&str, (Grid<Cell>, Vec<Direction>)> {
    let (input, grid) = parse_grid(input)?;
    let (input, path) = all_consuming(parse_path)(input)?;
    Ok((input, (grid, path)))
}

fn parse_path(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, r_dir) = many1(anychar.map(|c| {
        let x: Result<Direction, _> = c.try_into();
        x
    }))
    .parse(input)?;
    let dirs: Vec<Direction> = r_dir.into_iter().flat_map(|dir| dir.ok()).collect();
    Ok((input, dirs))
}
