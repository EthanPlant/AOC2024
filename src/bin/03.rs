use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use regex::Regex;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let input: String = reader
            .lines()
            .flatten()
            .collect();
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        Ok(regex.captures_iter(&input)
            .map(|m| {
                let a = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let b = m.get(2).unwrap().as_str().parse::<usize>().unwrap();
                a * b
            })
            .sum()
        )
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input: String = reader
            .lines()
            .flatten()
            .collect();
        let mut do_mul = true;
        let mut ans: usize = 0;
        let regex = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))").unwrap();
        for cap in regex.captures_iter(&input) {
            let instr = cap.get(1).unwrap().as_str();
            match instr {
                "do()" => do_mul = true,
                "don't()" => do_mul = false,
                _ => {
                    if do_mul {
                        let a = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
                        let b = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
                        ans += a * b;
                    }
                }
            }
        }

        Ok(ans)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
