use anyhow::*;
use itertools::enumerate;
use rustc_hash::FxHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "125 17";

fn parse_input<R: BufRead>(reader: R) -> Vec<usize> {
    reader
        .lines()
        .flatten()
        .map(|line| line
            .split_whitespace()
            .filter_map(|num_str| num_str.parse::<usize>().ok())
            .collect()
        )
        .next()
        .unwrap()
}

fn blink_once(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1]
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let divisor = 10usize.pow((stone.ilog10() + 1) / 2);
        return vec![stone / divisor, stone % divisor];
    }

    vec![stone * 2024]
}

fn blink_all(stones: FxHashMap<usize, usize>) -> FxHashMap<usize, usize> {
    let mut res = FxHashMap::default();

    for stone in stones {
        let new_stones = blink_once(stone.0);
        for new_stone in new_stones {
            res.entry(new_stone).and_modify(|count| *count += stone.1).or_insert(stone.1);
        }
    }

    res
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let stones = parse_input(reader);
        let mut freqs = stones
            .iter()
            .copied()
            .fold(FxHashMap::default(), |mut map, val| {
                map.entry(val)
                    .and_modify(|freq|*freq += 1)
                    .or_insert(1);
                map
            });
        for _ in 0..25 {
            freqs = blink_all(freqs);
        }

        Ok(freqs.values().sum())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let stones = parse_input(reader);
        let mut freqs = stones
            .iter()
            .copied()
            .fold(FxHashMap::default(), |mut map, val| {
                map.entry(val)
                    .and_modify(|freq|*freq += 1)
                    .or_insert(1);
                map
            });

        for _ in 0..75 {
            freqs = blink_all(freqs);
        }
        Ok(freqs.values().sum())
    }
        
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
