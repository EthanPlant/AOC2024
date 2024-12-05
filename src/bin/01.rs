use anyhow::*;
use itertools::Itertools;
use core::num;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn parse_input<R: BufRead>(reader: R) -> Vec<i32> {
    reader
        .lines()
        .flatten()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num_str| num_str.parse().ok())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn split_columns(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    numbers
        .into_iter()
        .enumerate()
        .partition_map(|(i, val)| {
            match i % 2 {
                0 => itertools::Either::Left(val),
                1 => itertools::Either::Right(val),
                _ => unreachable!()
            }
        })
    }

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let numbers = parse_input(reader);
        
        let (mut left, mut right) = split_columns(numbers);

        left.sort();
        right.sort();

        let result = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum::<i32>();

        Ok(result as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let numbers = parse_input(reader);
    
        let (mut left, mut right) = split_columns(numbers);

        left.sort();
        right.sort();

        let result: usize = left
            .iter()
            .map(|&l| (l as usize) * right.iter().filter(|&&x| x == l).count())
            .sum();

        Ok(result)
    }
    
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
