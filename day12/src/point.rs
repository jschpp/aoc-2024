use grid::Grid;
use std::ops::{self, Add, Index, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn new(a: usize, b: usize) -> Self {
        Self(a, b)
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point(value.0, value.1)
    }
}

impl From<&(usize, usize)> for Point {
    fn from(value: &(usize, usize)) -> Self {
        Point(value.0, value.1)
    }
}

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self[(index.0, index.1)]
    }
}

impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Point) -> &Self::Output {
        &self[*index]
    }
}

impl From<&Point> for (usize, usize) {
    fn from(value: &Point) -> Self {
        (value.0, value.1)
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        if rhs.0 >= 0 && rhs.1 >= 0 {
            Some(self + Point(rhs.0 as usize, rhs.1 as usize))
        } else {
            let l = self.0 as i32 + rhs.0;
            let c = self.1 as i32 + rhs.1;
            if l >= 0 && c >= 0 {
                Some(Self(l as usize, c as usize))
            } else {
                None
            }
        }
    }
}

impl Add<(i32, i32)> for &Point {
    type Output = Option<Point>;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        *self + rhs
    }
}

impl Add<&(i32, i32)> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: &(i32, i32)) -> Self::Output {
        self + *rhs
    }
}

impl Add<&(i32, i32)> for &Point {
    type Output = Option<Point>;

    fn add(self, rhs: &(i32, i32)) -> Self::Output {
        *self + *rhs
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Option<Point>;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        self + (-rhs.0, -rhs.1)
    }
}

impl Sub<(i32, i32)> for &Point {
    type Output = Option<Point>;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        *self - rhs
    }
}

impl Sub<&(i32, i32)> for Point {
    type Output = Option<Point>;

    fn sub(self, rhs: &(i32, i32)) -> Self::Output {
        self - *rhs
    }
}

impl Sub<&(i32, i32)> for &Point {
    type Output = Option<Point>;

    fn sub(self, rhs: &(i32, i32)) -> Self::Output {
        *self - *rhs
    }
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point(a.0 + b.0, a.1 + b.1)});

/// get the valid cardinal neighbours for a given position in the grid
///
/// checks wether the neighbours are in the grid or not
/// Will return an empty vector if there are no neighbours
pub fn get_cardinal_neighbours<T>(grid: &Grid<T>, p: &Point) -> Vec<Point> {
    const CARDINAL_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    CARDINAL_DIRECTIONS
        .iter()
        .flat_map(|dir| p + *dir)
        .filter(|p| grid.rows() > p.0 && grid.cols() > p.1)
        .collect()
}

// /// get all neighbours for a given position in the grid
// ///
// /// checks wether the neighbours are in the grid or not
// /// Will return an empty vector if there are no neighbours
// pub fn get_neighbours<T>(grid: &Grid<T>, p: &Point) -> Vec<Point> {
//     const DIRECTIONS: [(i32, i32); 8] = [
//         (-1, 0),
//         (1, 0),
//         (0, -1),
//         (0, 1),
//         (-1, -1),
//         (1, 1),
//         (1, -1),
//         (-1, 1),
//     ];
//     DIRECTIONS
//         .iter()
//         .flat_map(|dir| p + *dir)
//         .filter(|p| grid.rows() > p.0 && grid.cols() > p.1)
//         .collect()
// }
