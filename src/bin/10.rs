use anyhow::*;
use rustc_hash::FxHashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<usize>> {
    reader
        .lines()
        .flatten()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn is_valid_neighbor(map: &Vec<Vec<usize>>, start: (i32, i32), new_pos: (i32, i32)) -> bool {
    (new_pos.0 as usize) < map.len() && 
    (new_pos.1 as usize) < map[0].len() &&
    map[start.0 as usize][start.1 as usize] + 1 == map[new_pos.0 as usize][new_pos.1 as usize]
}

fn dfs(map: &Vec<Vec<usize>>, start: (i32, i32), part2: bool) -> usize {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut visited = FxHashSet::default();
    let mut stack = Vec::new();
    let mut count = 0;
    stack.push(start);

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();

        if visited.contains(&pos) && !part2 {
            continue;
        }

        visited.insert(pos);

        if map[pos.0 as usize][pos.1 as usize] == 9 {
            count += 1;
            continue;
        }

        for dir in dirs {
            let new_pos: (i32, i32) = (pos.0 + dir.0, pos.1 + dir.1);
            if is_valid_neighbor(map, pos, new_pos) {
                if !part2 &&visited.contains(&new_pos) {
                    continue;
                }
                stack.push(new_pos);
            }
        }
    }

    count
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_input(reader);

        let mut ans = 0;

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 0 {
                    ans += dfs(&map, (i as i32, j as i32), false);
                }
            }
        }

        Ok(ans)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_input(reader);

        let mut ans = 0;

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 0 {
                    ans += dfs(&map, (i as i32, j as i32), true);
                }
            }
        }

        Ok(ans)
    }
    
    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
