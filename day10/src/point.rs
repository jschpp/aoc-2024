use grid::Grid;
use std::ops::{self, Add, Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub usize, pub usize);

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
        if rhs.0 >= 0 && rhs.1 >= 0 {
            Some(self + Point(rhs.0 as usize, rhs.1 as usize))
        } else {
            let l = self.0 as i32 + rhs.0;
            let c = self.1 as i32 + rhs.1;
            if l >= 0 && c >= 0 {
                Some(Point(l as usize, c as usize))
            } else {
                None
            }
        }
    }
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point(a.0 + b.0, a.1 + b.1)});
