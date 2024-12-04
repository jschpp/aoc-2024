use std::ops::Add;

#[derive(Debug)]
pub struct Letter {
    pub letter: LetterOption,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub line: isize,
    pub letter: isize,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            line: self.line + rhs.line,
            letter: self.letter + rhs.letter,
        }
    }
}

impl Position {
    pub fn new(line: isize, letter: isize) -> Self {
        Self { line, letter }
    }
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Position {
            line: value.0,
            letter: value.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LetterOption {
    X,
    M,
    A,
    S,
    Invalid(char),
}

impl From<char> for LetterOption {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            val => Self::Invalid(val),
        }
    }
}
