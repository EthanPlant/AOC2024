#![feature(is_sorted)]

use anyhow::*;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    reader
        .lines()
        .flatten()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num_str| num_str.parse().ok())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let ascending = report.is_sorted_by(|a, b| {a < b && a.abs_diff(*b) <= 3});
    let descending = report.is_sorted_by(|a, b| {a > b && a.abs_diff(*b) <= 3});
    ascending || descending
}

fn is_report_safe_with_removal(report: &Vec<i32>) -> bool {
    if is_report_safe(report) {
        return true
    }

    report
        .iter()
        .cloned()
        .combinations(report.len() - 1)
        .any(|combination| is_report_safe(&combination))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let reports = parse_input(reader);
        
        Ok(
            reports
                .iter()
                .filter(|report| is_report_safe(report))
                .count()
        )
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let reports = reader
            .lines()
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|num_str| num_str.parse().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();

        Ok(
            reports
                .iter()
                .filter(|report| is_report_safe_with_removal(report))
                .count()
        )
    }
    
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
