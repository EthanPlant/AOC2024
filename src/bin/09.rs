use anyhow::*;
use core::fmt::Display;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "2333133121414131402";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
struct FileDescriptor {
    file_id: i64,
    pointer: usize,
    size: usize,
}

#[derive(Clone, Copy)]
enum Block {
    Filled(usize),
    Empty,
}

impl Block {
    fn is_empty(&self) -> bool {
        if let Block::Empty = &self {
            true
        } else {
            false
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Filled(val) => write!(f, "{}", val),
            Self::Empty => write!(f, "."),
        }
    }
}

struct Disk {
    blocks: Vec<Block>,
    files: Vec<FileDescriptor>,
    free_spaces: [BinaryHeap<Reverse<usize>>; 10],
}

impl Disk {
    fn get_first_empty(&self, start: usize) -> usize {
        self.blocks[start..].iter().position(|block| block.is_empty()).unwrap() + start
    }

    fn rearrange(&mut self) {
        let mut left = self.get_first_empty(0);
        let mut right = self.blocks.len() - 1;

        while left < right{
            if !self.blocks[right].is_empty() {
                self.blocks.swap(right, left);
                left = self.get_first_empty(left);
            }
            right -= 1;
        }
    }

    fn rearrange_files(&mut self) {
        for file in self.files.iter().rev() {
            let mut free_space = file.pointer;
            let mut old_size = 0;
            for i in file.size..10 {
                if let Some(&Reverse(free)) = self.free_spaces[i].peek() {
                    if free < free_space {
                        free_space = free;
                        old_size = i;
                    }
                }
            }

            if old_size != 0 {
                for i in 0..file.size {
                    self.blocks.swap(free_space + i, file.pointer + i);
                }
                self.free_spaces[old_size].pop();
                let new_free_space = Reverse(free_space + file.size);
                self.free_spaces[old_size - file.size].push(new_free_space);
            }
        }
    }

    fn calc_checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, v)| {
                match v {
                    Block::Filled(val) => i * val,
                    Block::Empty => 0
                }
            })
            .filter(|i| *i != 0)
            .sum()
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            write!(f, "{}", block).unwrap();
        }

        write!(f, "")
    }
}

fn parse_input<R: BufRead>(reader: R) -> Disk {
    let mut blocks = Vec::new();
    let mut files = Vec::new();
    let mut file_index = 0;
    let mut free_spaces: [BinaryHeap<Reverse<usize>>; 10] = [
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
    ];

    for line in reader.lines() {
        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            let val = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                for _ in 0..val {
                    blocks.push(Block::Filled(file_index));
                }
                files.push(FileDescriptor {
                    file_id: file_index as i64,
                    pointer: blocks.len() - val,
                    size: val,
                });
                file_index += 1;
            } else {
                for _ in 0..val {
                    blocks.push(Block::Empty);
                }
                free_spaces[val].push(Reverse(blocks.len() - val));
            }
        }
    }

    Disk {
        blocks,
        files,
        free_spaces,
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut disk = parse_input(reader);
        disk.rearrange();

        Ok(disk.calc_checksum())
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut disk = parse_input(reader);
        disk.rearrange_files();

        Ok(disk.calc_checksum())
    }
    
    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
