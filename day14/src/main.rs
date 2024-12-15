use std::collections::HashMap;

use cached::proc_macro::cached;
use glam::{ivec2, IVec2};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const FIELD_HIGHT: i32 = 103;
const FIELD_WIDTH: i32 = 101;
const SECONDS: usize = 100;

fn main() {
    let input = include_str!("../input.txt");
    let (input, r) = parse(input).unwrap();
    assert!(input.is_empty());
    part2(&r);
}

#[allow(dead_code)]
fn part1(robots: &[Robot]) {
    let robots = robots
        .into_par_iter()
        .map(|r| {
            let new_r = move_robot(r.position, r.velocity, SECONDS);
            Robot {
                position: new_r,
                velocity: r.velocity,
            }
        })
        .collect::<Vec<_>>();
    print_field(&robots);
    println!("part1: {}", get_safety_factor(&robots));
}

fn part2(robots: &[Robot]) {
    if let Some(step_count) = (0..usize::MAX).find(|step_count| {
        let robots = robots
            .into_par_iter()
            .map(|r| {
                let new_r = move_robot(r.position, r.velocity, *step_count);
                Robot {
                    position: new_r,
                    velocity: r.velocity,
                }
            })
            .collect::<Vec<_>>();

        // this seems to work?
        // not sure if on every input...
        let c = counter(&robots);
        robots.len() == c.len()
    }) {
        let robots = robots
            .into_par_iter()
            .map(|r| {
                let new_r = move_robot(r.position, r.velocity, step_count);
                Robot {
                    position: new_r,
                    velocity: r.velocity,
                }
            })
            .collect::<Vec<_>>();
        print_field(&robots);
        println!("part2: {}", step_count);
    }
}

fn get_safety_factor(robots: &[Robot]) -> usize {
    let q = robots.iter().fold((0, 0, 0, 0), |mut acc, r| {
        if let Some(q) = r.get_quadrant() {
            match q {
                Quadrant::UpperLeft => acc.0 += 1,
                Quadrant::UpperRight => acc.1 += 1,
                Quadrant::LowerLeft => acc.2 += 1,
                Quadrant::LowerRight => acc.3 += 1,
            }
        };
        acc
    });
    q.0 * q.1 * q.2 * q.3
}

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

enum Quadrant {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

impl Robot {
    fn get_quadrant(&self) -> Option<Quadrant> {
        let mid_x = FIELD_WIDTH / 2;
        let mid_y = FIELD_HIGHT / 2;

        // upper left
        if self.position.x < mid_x && self.position.y < mid_y {
            return Some(Quadrant::UpperLeft);
        };

        // upper right
        if self.position.x > mid_x && self.position.y < mid_y {
            return Some(Quadrant::UpperRight);
        };

        // lower left
        if self.position.x < mid_x && self.position.y > mid_y {
            return Some(Quadrant::LowerLeft);
        };

        // lower right
        if self.position.x > mid_x && self.position.y > mid_y {
            return Some(Quadrant::LowerRight);
        };
        None
    }
}

fn parse_vec(input: &str) -> IResult<&str, IVec2> {
    separated_pair(complete::i32, tag(","), complete::i32)
        .map(|(x, y)| ivec2(x, y))
        .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Robot> {
    let (input, position) = preceded(tag("p="), parse_vec)(input)?;
    let (input, velocity) = preceded(tag(" v="), parse_vec)(input)?;
    Ok((input, Robot { position, velocity }))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list0(line_ending, parse_line)(input)
}

#[cached]
fn move_robot(position: IVec2, velocity: IVec2, step_count: usize) -> IVec2 {
    // if step_count == 0 {
    //     return position;
    // }
    // if new_x < 0 {
    //     new_x += FIELD_WIDTH;
    // } else if new_x >= FIELD_WIDTH {
    //     new_x -= FIELD_WIDTH;
    // }
    let new_x = (position.x + (step_count as i32 * velocity.x)).rem_euclid(FIELD_WIDTH);
    let new_y = (position.y + (step_count as i32 * velocity.y)).rem_euclid(FIELD_HIGHT);

    let next_position = ivec2(new_x, new_y);
    // move_robot(next_position, velocity, step_count - 1)
    next_position
}

fn print_field(robots: &[Robot]) {
    let robots = counter(robots);
    let dot = String::from(".");
    for y in 0..FIELD_HIGHT {
        for x in 0..FIELD_WIDTH {
            let ch = robots
                .get(&ivec2(x, y))
                .map(|x| x.to_string())
                .unwrap_or(dot.clone());
            print!("{ch}");
        }
        println!();
    }
}

fn counter(input: &[Robot]) -> HashMap<IVec2, usize> {
    let mut h = HashMap::default();
    for robot in input {
        h.entry(robot.position).and_modify(|e| *e += 1).or_insert(1);
    }
    h
}
