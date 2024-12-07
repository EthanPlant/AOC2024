use anyhow::*;
use rustc_hash::FxHashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Dir {
    pub fn get_vector(&self) -> (i32, i32) {
        match self {
            Dir::UP => (0, 1),
            Dir::DOWN => (0, -1),
            Dir::LEFT => (-1, 0),
            Dir::RIGHT => (1, 0),
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: (i32, i32),
    dir: Dir,
}

impl Guard {
    pub fn get_visited_positions(&mut self, map: &Map) -> FxHashSet<(i32, i32)> {
        let mut visited = FxHashSet::default();
        visited.insert(self.position);
        while self.is_in_map(map) {
            let vec = self.dir.get_vector();
            let new_pos = (self.position.0 + vec.0, self.position.1 + vec.1);
            if map.obstacles.contains(&new_pos) {
                self.rotate();
            } else {
                self.position = new_pos;
                if !visited.contains(&self.position) && self.is_in_map(map) {
                    visited.insert(self.position);
                }
            }
        }

        visited
    }

    pub fn check_if_loop(&mut self, map: &Map) -> bool {
        let start_pos = self.position;
        let mut turns = FxHashSet::default();
        while self.is_in_map(map) {
            let vec = self.dir.get_vector();
            let new_pos = (self.position.0 + vec.0, self.position.1 + vec.1);
            if map.obstacles.contains(&new_pos) {
                if turns.contains(&(new_pos, self.dir)) {
                    self.position = start_pos;
                    self.dir = Dir::UP;
                    return true
                }
                turns.insert((new_pos, self.dir));
                self.rotate();
            } else {
                self.position = new_pos;
            }
        }
        self.position = start_pos;
        self.dir = Dir::UP;
        false
    }

    fn rotate(&mut self) {
        match self.dir {
            Dir::UP => self.dir = Dir::RIGHT,
            Dir::DOWN => self.dir = Dir::LEFT,
            Dir::LEFT => self.dir = Dir::UP,
            Dir::RIGHT => self.dir = Dir::DOWN,
        }
    }

    fn is_in_map(&self, map: &Map) -> bool {
        self.position.0 >= 0 && self.position.0 < map.width as i32 &&
            self.position.1 >= 0 && self.position.1 < map.height as i32
    }
}

#[derive(Debug)]
struct Map {
    obstacles: FxHashSet<(i32, i32)>,
    width: usize,
    height: usize,
}

fn parse_input<R: BufRead>(reader: R) -> (Guard, Map) {
    let input: Vec<Vec<_>> = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();

    let width = input[0].len();
    let height = input.len();

    let mut obstacles = FxHashSet::default();
    let mut guard = None;

    for i in 0..height {
        for j in 0..width {
            let c = input[i][j];
            if c == '#' {
                obstacles.insert((j as i32, height as i32 - i as i32 - 1));
            } else if c == '^' {
                guard = Some(Guard {
                    position: (j as i32, height as i32 - i as i32 - 1),
                    dir: Dir::UP,
                });
            }
        }
    }

    let map = Map {
        obstacles,
        width,
        height
    };

    (guard.unwrap(), map)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut guard, map) = parse_input(reader);

        let mut visited = FxHashSet::default();
        visited.insert(guard.position);

        Ok(guard.get_visited_positions(&map).len())
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (mut guard, mut map) = parse_input(reader);
        let start_pos = guard.position;

        let positions = guard.get_visited_positions(&map);
        guard.position = start_pos;
        guard.dir = Dir::UP;

        let mut count = 0;

        for position in positions {
            if position != start_pos {
                map.obstacles.insert(position);
                if guard.check_if_loop(&map) {
                    count += 1;
                }
                map.obstacles.remove(&position);
        }
        }

        Ok(count)
    }
    
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
