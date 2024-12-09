use std::{
    fmt::{Debug, Display},
    ops::AddAssign,
    str::FromStr,
};

fn main() {
    let input = include_str!("../input.txt");
    let layout = DiskLayout::from_str(input).expect("layout should be valid");
    let mut part1 = layout.clone();
    part1.compact_part1();
    println!("part1 checksum {}", part1.checksum());

    let mut part2 = layout.clone();
    part2.compact_part2();
    println!("part2 checksum {}", part2.checksum());
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
            // move pointer to the next free slot
            if self.layout[left_pointer] != BlockType::Free {
                left_pointer += 1;
                continue;
            }

            // move pointer to the next slot which contains a file
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
            .expect("layout should contain at least one block");
        last_found_id.add_assign(1); // since the loops starts by running a sub we need to add one here
        while let Some(new_id) = last_found_id.checked_sub(1) {
            last_found_id = new_id;

            let next_block = self.find_block_info(last_found_id);
            if let Some(free_space_start) = self.find_free_block(next_block.size, next_block.start)
            {
                for i in 0..next_block.size {
                    self.layout.swap(free_space_start + i, next_block.end - i);
                }
            }
        }
    }

    fn find_free_block(&self, length: usize, max_idx: usize) -> Option<usize> {
        self.layout[0..max_idx] // only need to check left from the start of our block
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

    fn find_block_info(&self, id: usize) -> BlockInfo {
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

impl Display for DiskLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.layout.iter() {
            write!(f, "{}", block)?
        }
        Ok(())
    }
}

impl FromStr for DiskLayout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<usize> = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|x| x as usize)
            .collect();
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
