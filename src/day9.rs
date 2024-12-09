use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Free,
    Used { id: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct File {
    num_blocks: usize,
    block: Block,
}

#[aoc_generator(day9, part1)]
pub fn gen_blocks(input: &str) -> Vec<Block> {
    input
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as usize)
        .enumerate()
        .flat_map(|(i, n)| {
            let block = if (i % 2) == 0 {
                // It's a file
                Block::Used { id: i / 2 }
            } else {
                Block::Free
            };
            std::iter::repeat_n(block, n)
        })
        .collect()
}

#[aoc_generator(day9, part2)]
pub fn gen_files(input: &str) -> Vec<File> {
    input
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as usize)
        .enumerate()
        .map(|(i, n)| {
            let block = if (i % 2) == 0 {
                // It's a file
                Block::Used { id: i / 2 }
            } else {
                Block::Free
            };
            File {
                block,
                num_blocks: n,
            }
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(blocks: &[Block]) -> usize {
    let mut compacted = blocks.to_vec();
    for b in (0..blocks.len()).rev() {
        let Some(idx) = compacted.iter().position(|cblock| *cblock == Block::Free) else {
            break;
        };
        if idx >= b {
            break;
        }
        compacted.swap(b, idx);
    }
    compacted
        .into_iter()
        .enumerate()
        .map(|(b, block)| match block {
            Block::Free => 0,
            Block::Used { id } => b * id,
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(files: &[File]) -> usize {
    let mut compacted = files.to_vec();
    for file in files.iter().rev() {
        if file.block == Block::Free {
            continue;
        }
        let Some(idx) = compacted
            .iter()
            .position(|cfile| cfile.num_blocks >= file.num_blocks && cfile.block == Block::Free)
        else {
            // Just because we couldn't be placed doesn't mean others can't
            continue;
        };
        let fc = compacted.iter().position(|f| f == file).unwrap();
        // Check that we're moving in the right direction
        if idx >= fc {
            continue;
        }
        // Replace the file with free space. Keep in mind it might have moved
        // so we need to find it
        compacted[fc] = File {
            num_blocks: file.num_blocks,
            block: Block::Free,
        };
        // Decrease the free size
        compacted[idx].num_blocks -= file.num_blocks;
        // Insert a new file before (or replace if possible)
        if compacted[idx].num_blocks == 0 {
            compacted[idx] = *file;
        } else {
            compacted.insert(idx, *file);
        }
    }

    compacted
        .into_iter()
        .flat_map(|file| {
            let n = match file.block {
                Block::Free => 0,
                Block::Used { id } => id,
            };
            std::iter::repeat_n(n, file.num_blocks)
        })
        .enumerate()
        .map(|(i, n)| i * n)
        .sum()
}
