use itertools::Itertools;
use std::ops;
#[macro_use]
extern crate impl_ops;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

fn main() {
    let input = include_str!("../input.txt");
    let grid = Grid::from_str(input).expect("valid grid");

    let mut part1 = grid.clone();
    part1.find_antinodes_part1();
    println!("part1: {}", part1.count());

    let mut part2 = grid.clone();
    part2.find_antinodes_part2();
    println!("part2: {}", part2.count());
}

#[derive(Debug, Clone)]
struct Grid {
    data: HashMap<Point, char>,
    antinodes: HashSet<Point>,
    found_towers: HashSet<char>,
    line_max: isize,
    char_max: isize,
}

impl Grid {
    fn add_antinode(&mut self, p: Point) {
        if self.is_in_bounds(p) {
            self.antinodes.insert(p);
        }
    }

    fn find_antinodes_part1(&mut self) {
        let mut to_add = Vec::default();
        for tower_type in self.found_towers.iter() {
            for tuple in self
                .data
                .iter()
                .filter(|(_coord, c)| *c == tower_type)
                .tuple_combinations::<((&Point, &char), (&Point, &char))>()
            {
                let ((a, _), (b, _)) = tuple;
                let vector = a - b;
                to_add.push(a + vector);
                to_add.push(b - vector);
            }
        }
        to_add.into_iter().for_each(|p| {
            self.add_antinode(p);
        });
    }

    fn is_in_bounds(&self, p: Point) -> bool {
        p.0 <= self.line_max && p.1 <= self.char_max && p.0 >= 0 && p.1 >= 0
    }

    fn find_antinodes_part2(&mut self) {
        let mut to_add = Vec::default();
        for tower_type in self.found_towers.iter() {
            for tuple in self
                .data
                .iter()
                .filter(|(_coord, c)| *c == tower_type)
                .tuple_combinations::<((&Point, &char), (&Point, &char))>()
            {
                let ((a, _), (b, _)) = tuple;

                // first add both towers since they are themselve in line
                to_add.push(*a);
                to_add.push(*b);

                // then add the difference up and add the antinode as long as the new antinode is still inside the bounds
                let initial_vector = a - b;
                let mut vector = initial_vector;
                while self.is_in_bounds(a + vector) {
                    to_add.push(a + vector);
                    vector = vector + initial_vector;
                }

                let mut vector = initial_vector;
                while self.is_in_bounds(b - vector) {
                    to_add.push(b - vector);
                    vector = vector + initial_vector;
                }
            }
        }
        to_add.into_iter().for_each(|p| {
            self.add_antinode(p);
        });
    }

    fn count(&self) -> usize {
        self.antinodes.len()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g = Grid {
            data: HashMap::default(),
            antinodes: HashSet::default(),
            found_towers: HashSet::default(),
            line_max: 0,
            char_max: 0,
        };
        for (line_idx, line) in s.lines().enumerate() {
            if line_idx as isize >= g.line_max {
                g.line_max = line_idx as isize
            }
            for (char_idx, c) in line.chars().enumerate() {
                if c != '.' {
                    g.data
                        .insert(Point(line_idx as isize, char_idx as isize), c);
                    g.found_towers.insert(c);
                }
                if char_idx as isize >= g.char_max {
                    g.char_max = char_idx as isize
                }
            }
        }
        Ok(g)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line_idx in 0..=self.line_max {
            for char_idx in 0..=self.char_max {
                let coord = Point(line_idx, char_idx);
                if self.data.contains_key(&coord) {
                    write!(f, "{}", self.data[&Point(line_idx, char_idx)])?;
                } else if self.antinodes.contains(&coord) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(isize, isize);

impl_op_ex!(-|a: &Point, b: &Point| -> Point { Point(a.0 - b.0, a.1 - b.1) });
impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point(a.0 + b.0, a.1 + b.1)});
