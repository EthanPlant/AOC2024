use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

fn test_word(grid: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let chars = ['X', 'M', 'A', 'S'];

    if grid[x][y] != chars[0] {
        return 0;
    }

    let mut count = 0;

    let dirs: [(i32, i32); 8] = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1,0),
        (-1, -1)
    ];

    for dir in dirs {
        let mut curr_x = (x as i32 + dir.0) as usize;
        let mut curr_y = (y as i32 + dir.1) as usize;
        let mut i = 1;

        while i < chars.len() {
            if curr_x >= grid.len() || curr_y >= grid[0].len() || grid[curr_x][curr_y] != chars[i] {
                break;
            }

            curr_x = (curr_x as i32 + dir.0) as usize;
            curr_y = (curr_y as i32 + dir.1) as usize;
            i += 1;
        }

        if i == chars.len() {
            count += 1;
        }
    }

    count
}

fn check_xmas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x < 1 || x >= grid.len() - 1 || y < 1 || y >= grid.len() - 1 || grid[x][y] != 'A' {
        return false;
    }

    let diag = grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S' ||
        grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M';
    let anti_diag = grid[x - 1][y + 1] == 'M' && grid[x + 1][y - 1] == 'S' ||
        grid[x - 1][y + 1] == 'S' && grid[x + 1][y - 1] == 'M';

    diag && anti_diag
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let input = reader
            .lines()
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(|row| row.chars())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let word = "XMAS";

        let mut count = 0;

        for i in 0..input.len() {
            for j in 0..input[i].len() {
                count += test_word(&input, i, j);
            }
        }

        Ok(count)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader
            .lines()
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(|row| row.chars())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut count = 0;

        let rows = input.len();
        let cols = input[0].len();

        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                if check_xmas(&input, i, j) {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
    
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
