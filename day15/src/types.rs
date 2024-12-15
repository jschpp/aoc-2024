use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Crate,
    Robot,
    Empty,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '@' => Cell::Robot,
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'O' => Cell::Crate,
            c => panic!("found {} hex: {}", c, c as u8),
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Robot => write!(f, "@"),
            Cell::Empty => write!(f, "."),
            Cell::Crate => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum BiggerCell {
    Wall,
    CrateLeft,
    CrateRight,
    #[default]
    Empty,
    Robot,
}

impl Debug for BiggerCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BiggerCell::Wall => write!(f, "#"),
            BiggerCell::CrateLeft => write!(f, "["),
            BiggerCell::CrateRight => write!(f, "]"),
            BiggerCell::Empty => write!(f, "."),
            BiggerCell::Robot => write!(f, "@"),
        }
    }
}
