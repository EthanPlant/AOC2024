use anyhow::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "13"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize)
}

fn solve_machine(machine: &Machine, offset: isize) -> isize {
    let prize = (machine.prize.0 + offset, machine.prize.1 + offset);
    let det = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;
    let a = (prize.0 * machine.b.1 - prize.1 * machine.b.0) / det;
    let b = (machine.a.0 * prize.1 - machine.a.1 * prize.0) / det;
    if (machine.a.0 * a + machine.b.0 * b, machine.a.1 * a + machine.b.1 * b) == (prize.0, prize.1) {
        a * 3 + b
    } else {
        0
    }
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Machine> {
    let a_regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut machines = Vec::new();
    let mut a = None;
    let mut b = None;
    let mut prize = None;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if a.is_none() {
            let cap = a_regex.captures(line.as_str()).unwrap();
            a = Some((cap.get(1).unwrap().as_str().parse().unwrap(), cap.get(2).unwrap().as_str().parse().unwrap()));
        } else if b.is_none() {
            let cap = b_regex.captures(line.as_str()).unwrap();
            b = Some((cap.get(1).unwrap().as_str().parse().unwrap(), cap.get(2).unwrap().as_str().parse().unwrap()));
        } else if prize.is_none() {
            let cap = prize_regex.captures(line.as_str()).unwrap();
            prize = Some((cap.get(1).unwrap().as_str().parse().unwrap(), cap.get(2).unwrap().as_str().parse().unwrap()));
            machines.push(Machine {
                a: a.unwrap(),
                b: b.unwrap(),
                prize: prize.unwrap(),
            });
            a = None;
            b = None;
            prize = None;
        }
    }

    machines
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let machines = parse_input(reader);
        Ok(machines
            .into_iter()
            .map(|machine| solve_machine(&machine, 0) as usize)
            .sum()
        )
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<u128> {
        let machines = parse_input(reader);
        println!("{:?}", machines);
        Ok(machines
            .into_iter()
            .map(|machine| solve_machine(&machine, 10000000000000) as u128)
            .sum()
        )
    }
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
