use anyhow::*;
use grid::{Dir, Grid, Vector};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

const WIDTH: i32 = 70;
const HEIGHT: i32 = 70;
const BYTES: usize = 1024;

#[derive(Debug)]
struct Memory {
    bytes: Vec<Vector>
}

impl Memory {
    fn bfs(&self) -> Option<usize> {
        let mut score = None;
        let mut frontier = VecDeque::new();
        let mut visited = FxHashSet::default();

        frontier.push_back((Vector(0, 0), 0));
        visited.insert(Vector(0, 0));

        while let Some((curr, curr_score)) = frontier.pop_front() {
            if curr == Vector(WIDTH, HEIGHT) {
                score = Some(curr_score);
                break;
            }

            for dir in Dir::iter() {
                if self.can_move(curr, dir) && !visited.contains(&(curr + dir.into())) {
                    frontier.push_back((curr + dir.into(), curr_score + 1));
                    visited.insert(curr + dir.into());
                }
            }
        }

        score
    }

    fn binary_search(&mut self, bytes: &Vec<Vector>) -> usize {
        let mut left = 0;
        let mut right = bytes.len();

        while right - left > 1 {
            let mid = left + (right - left) / 2;
            self.bytes = bytes.iter().cloned().take(mid).collect();
            if self.bfs().is_some() {
                left = mid;
            } else {
                right = mid;
            }
        }

        left
    }
}

impl Grid for Memory {
    fn can_move(&self, pos: Vector, dir: Dir) -> bool {
        let new_pos = pos + dir.into();
        new_pos.0 >= 0 && new_pos.0 <= HEIGHT && 
        new_pos.1 >= 0 && new_pos.1 <= WIDTH &&
        !self.bytes.contains(&new_pos)
    }

    fn is_in_map(&self, _: Vector) -> bool {
        todo!()
    }

    fn new_from_map(_: &str) -> Self {
        todo!()
    }
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vector> {
    reader
        .lines()
        .flatten()
        .map(|line| {
            line
                .split(',')
                .map(|num| num.parse().unwrap()).collect::<Vec<_>>()
        })
        .map(|coords| Vector(coords[1], coords[0]))
        .collect()
        
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = parse_input(reader);
        let memory = Memory {
            bytes: input.into_iter().take(BYTES).collect(),
        };
        Ok(memory.bfs().unwrap())
    }

    //assert_eq!(22, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<Vector> {
        let input = parse_input(reader);
        let mut memory = Memory {
            bytes: Vec::new(),
        };
        Ok(input[memory.binary_search(&input)])
    }
    
   //assert_eq!(Vector(1, 6), part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {:?}", result);

    Ok(())
}
