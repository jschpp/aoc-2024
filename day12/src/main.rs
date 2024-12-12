use grid::Grid;
use itertools::Itertools;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};
#[macro_use]
extern crate impl_ops;
mod point;
use point::{checked_idx, get_cardinal_neighbours, Point};

fn main() {
    let input = include_str!("../input.txt");
    let g: Grid<char> = parse(input);

    // find all regions
    let region = find_region(&g);

    let value: usize = region.iter().map(|r| r.get_value_part1(&g)).sum();
    println!("part1 {value}");

    let value: usize = region.iter().map(|r| r.get_value_part2(&g)).sum();
    println!("part2 {value}");
}

fn parse(input: &str) -> Grid<char> {
    let width = input
        .lines()
        .next()
        .expect("there should be at least one row")
        .chars()
        .count();
    let data = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<Vec<_>>();
    Grid::from_vec(data, width)
}

#[derive(Debug, Default, Clone)]
struct Region {
    data: HashSet<Point>,
}

#[allow(dead_code)]
impl Region {
    fn add(&mut self, p: Point) {
        self.data.insert(p);
    }

    fn get_area(&self) -> usize {
        self.data.len()
    }

    fn get_fence_length(&self, g: &Grid<char>) -> usize {
        let mut result = 0_usize;
        for point in self.data.iter() {
            let count = get_cardinal_neighbours(g, point)
                .iter()
                .filter(|x| self.data.contains(x))
                .count();
            result += 4 - count;
        }
        result
    }

    fn get_value_part1(&self, g: &Grid<char>) -> usize {
        self.get_area() * self.get_fence_length(g)
    }

    fn get_letter(&self, g: &Grid<char>) -> char {
        let point = self.data.iter().next().expect("not empty");
        g[point]
    }

    fn get_value_part2(&self, g: &Grid<char>) -> usize {
        let own_letter = Some(self.get_letter(g));
        let mut corners = 0_usize;
        for p in self.data.iter() {
            let n = (p + (-1, 0)).and_then(|p| checked_idx(p, g));
            let s = (p + (1, 0)).and_then(|p| checked_idx(p, g));
            let w = (p + (0, -1)).and_then(|p| checked_idx(p, g));
            let e = (p + (0, 1)).and_then(|p| checked_idx(p, g));
            let ne = (p + (-1, 1)).and_then(|p| checked_idx(p, g));
            let nw = (p + (-1, -1)).and_then(|p| checked_idx(p, g));
            let se = (p + (1, 1)).and_then(|p| checked_idx(p, g));
            let sw = (p + (1, -1)).and_then(|p| checked_idx(p, g));

            // convex corners

            // upper left corner
            if n != own_letter && w != own_letter {
                corners += 1;
            }

            // lower left corner
            if s != own_letter && w != own_letter {
                corners += 1;
            }

            // lower right corner
            if s != own_letter && e != own_letter {
                corners += 1;
            }

            // upper right corner
            if n != own_letter && e != own_letter {
                corners += 1;
            }

            // concave corners

            // corner check
            // |  X  | A |
            // |  -  +   |
            // |  A    p |
            // we are at p and to check for a convex corner both neigbhours in the cardinal directions
            // __must__ be within our region (checking if they are the same and in our region)
            // then the corner marked with X must not be the same region as ours (could be None could be Some(whatever))
            // for every other direction rotate

            // upper left concave
            if w == n && w == own_letter && nw != own_letter {
                corners += 1;
            }

            // lower left concave
            if w == s && w == own_letter && sw != own_letter {
                corners += 1;
            }

            // lower right concave
            if e == s && e == own_letter && se != own_letter {
                corners += 1;
            }

            // upper right concave
            if e == n && e == own_letter && ne != own_letter {
                corners += 1;
            }
        }
        corners * self.get_area()
    }
}

impl From<HashSet<Point>> for Region {
    fn from(value: HashSet<Point>) -> Self {
        Self { data: value }
    }
}

fn find_region(grid: &Grid<char>) -> Vec<Region> {
    let mut seen_points: HashSet<Point> = HashSet::default();
    let mut found_regions: Vec<Region> = Vec::default();
    for point in (0..grid.rows())
        .cartesian_product(0..grid.cols())
        .map(|(line, col)| Point(line, col))
    {
        if seen_points.contains(&point) {
            // this region was already found
            continue;
        }

        let new_region: Arc<RwLock<HashSet<Point>>> = Arc::new(RwLock::new(HashSet::default()));
        flood_fill_region(new_region.clone(), point, grid);

        // putting all found points in seen
        let points = new_region.read().unwrap().clone();
        seen_points.extend(points.clone());

        // putting the region in a hashmap with
        let region = Region::from(points);
        found_regions.push(region);
    }
    found_regions
}

fn flood_fill_region(current_region: Arc<RwLock<HashSet<Point>>>, next: Point, grid: &Grid<char>) {
    current_region.write().expect("not poisoned").insert(next);
    get_cardinal_neighbours(grid, &next)
        .into_iter()
        .filter(|coord| {
            grid[*coord] == grid[next] && !current_region.read().expect("read").contains(coord)
        })
        .for_each(|p| flood_fill_region(current_region.clone(), p, grid));
}
