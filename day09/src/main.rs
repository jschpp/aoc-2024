use std::{
    fmt::{Debug, Display},
    num::IntErrorKind,
    ops::AddAssign,
    str::FromStr,
};

fn main() {
    let input = include_str!("../input.txt");
    let layout = DiskLayout::from_str(input).expect("valid");
    let mut part1 = layout.clone();
    part1.compact_part1();
    println!("part1 checksum {}", part1.checksum());

    let mut part2 = layout.clone();
    // let mut part2 = DiskLayout::from_str("2333133121414131402").expect("valid layout");
    part2.compact_part2();
    println!("part1 checksum {}", part2.checksum());
}

#[derive(Debug, Clone)]
struct DiskLayout {
    layout: Vec<BlockType>,
}

impl DiskLayout {
    fn compact_part1(&mut self) {
        let mut left_pointer: usize = 0;
        let mut right_pointer: usize = self.layout.len() - 1;
        while left_pointer < right_pointer {
            if self.layout[left_pointer] != BlockType::Free {
                left_pointer += 1;
                continue;
            }
            if self.layout[right_pointer] == BlockType::Free {
                right_pointer -= 1;
                continue;
            }
            self.layout.swap(left_pointer, right_pointer);
        }
    }

    fn compact_part2(&mut self) {
        let mut last_found_id: usize = *self
            .layout
            .iter()
            .rev()
            .filter_map(|b| match b {
                BlockType::Free => None,
                BlockType::File(val) => Some(val),
            })
            .next()
            .expect("at least one block");
        last_found_id.add_assign(1);
        while let Some(id) = last_found_id.checked_sub(1) {
            last_found_id = id;
            let current_block = self.find_next_block_to_move(id);
            if let Some(free_space) = self.find_free_block(current_block.size) {
                let mut left_pointer: usize = free_space;
                let mut right_pointer: usize = current_block.end;
                if left_pointer >= current_block.start {
                    continue;
                }
                for _ in 0..current_block.size {
                    self.layout.swap(left_pointer, right_pointer);
                    left_pointer += 1;
                    right_pointer -= 1;
                }
            }
        }
    }

    fn find_free_block(&self, length: usize) -> Option<usize> {
        self.layout
            .windows(length)
            .enumerate()
            .filter_map(|(start, v)| {
                if v.iter().all(|x| *x == BlockType::Free) {
                    Some(start)
                } else {
                    None
                }
            })
            .next()
    }

    fn find_next_block_to_move(&self, id: usize) -> BlockInfo {
        let x = self
            .layout
            .iter()
            .enumerate()
            .rev()
            .filter(|(_idx, b)| **b == BlockType::File(id))
            .rev()
            .collect::<Vec<_>>();
        BlockInfo::new(x[0].0, x[x.len() - 1].0)
    }

    fn checksum(&self) -> usize {
        self.layout
            .iter()
            .enumerate()
            .filter_map(|(idx, b)| match b {
                BlockType::Free => None,
                BlockType::File(id) => Some(idx * id),
            })
            .sum()
    }
}

#[derive(Debug)]
struct BlockInfo {
    start: usize,
    end: usize,
    size: usize,
}

impl BlockInfo {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            size: 1 + end - start,
        }
    }
}

impl Display for DiskLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.layout.iter() {
            write!(f, "{}", block)?
        }
        Ok(())
    }
}

impl FromStr for DiskLayout {
    type Err = IntErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .chars()
            .map(|c| c.to_digit(10).expect("valid num") as usize)
            .collect::<Vec<usize>>();
        let mut layout: Vec<BlockType> = Vec::default();
        let mut found_files = 0_usize;
        for (idx, block_size) in v.iter().enumerate() {
            let b = match idx % 2 {
                0 => {
                    let b = BlockType::File(found_files);
                    found_files += 1;
                    b
                }
                1 => BlockType::Free,
                _ => unreachable!(),
            };
            layout.append(&mut vec![b; *block_size]);
        }
        Ok(DiskLayout { layout })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    Free,

    /// File (id)
    File(usize),
}

impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::Free => write!(f, "."),
            BlockType::File(id) => write!(f, "{id}"),
        }
    }
}
