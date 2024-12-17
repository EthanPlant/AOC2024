use anyhow::*;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn step(&mut self, steps: i32) {
        self.pos = ((self.pos.0 + self.vel.0 * steps).rem_euclid(101), (self.pos.1 + (self.vel.1 * steps)).rem_euclid(103));
    }
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Robot> {
    let mut robots = Vec::new();
    let re = Regex::new(r"p=((\d+),(\d+)) v=((-?\d+),(-?\d+))").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let cap = re.captures(&line).unwrap();
        robots.push(Robot {
            pos: (cap.get(2).unwrap().as_str().parse().unwrap(), cap.get(3).unwrap().as_str().parse().unwrap()),
            vel: (cap.get(5).unwrap().as_str().parse().unwrap(), cap.get(6).unwrap().as_str().parse().unwrap())
        });
    }

    robots
}

fn get_safety_factor(robots: &Vec<Robot>) -> usize {
    let mid_x = 101 / 2;
    let mid_y = 103 / 2;

    let mut quad_1 = 0;
    let mut quad_2 = 0;
    let mut quad_3 = 0;
    let mut quad_4 = 0;

    for robot in robots {
        if robot.pos.0 < mid_x && robot.pos.1 < mid_y {
            quad_1 += 1;
        } else if robot.pos.0 > mid_x && robot.pos.1 < mid_y {
            quad_2 += 1;
        } else if robot.pos.0 < mid_x && robot.pos.1 > mid_y {
            quad_3 += 1;
        } else if robot.pos.0 > mid_x && robot.pos.1 > mid_y {
            quad_4 += 1;
        }
    }

    quad_1 * quad_2 * quad_3 * quad_4
}

fn are_all_unique(robots: &Vec<Robot>) -> bool {
    robots
        .iter()
        .map(|robot| robot.pos)
        .all_unique()
}

fn print_robots(robots: &Vec<Robot>) {
    for i in 0..103 {
        for j in 0..101 {
            print!(
                "{}",
                if robots.iter().find(|robot| robot.pos == (j, i)).is_some() {
                    "#"
                } else {
                    "."
                }
            )
        }
        println!()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut robots = parse_input(reader);
        for i in 0..robots.len() {
            robots[i].step(100);
        }
        Ok(get_safety_factor(&robots))
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut robots = parse_input(reader);
        // print_robots(&robots);
        // println!();
        let mut steps = 0;
        while !are_all_unique(&robots) {
            for robot in &mut robots {
                robot.step(1);
            }

            steps += 1;
        }

        print_robots(&robots);

        Ok(steps)
    }
        
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
