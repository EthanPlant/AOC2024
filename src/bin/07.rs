use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

#[derive(Debug)]
struct Equation(usize, Vec<usize>);

// fn check_equation_valid(equation: &Vec<usize>) -> bool {
//     if equation.len() == 2 {
//         return equation[1] == equation[0];
//     }
//     if equation[1] > equation[0] {
//         return false;
//     }

//     let mut add_equation = equation.clone();
//     let mut mul_equation = equation.clone();

//     add_equation.remove(2);
//     add_equation.remove(1);
//     add_equation.insert(1, equation[1] + equation[2]);

//     mul_equation.remove(2);
//     mul_equation.remove(1);
//     mul_equation.insert(1, equation[1] * equation[2]);

//     check_equation_valid(&add_equation) || check_equation_valid(&mul_equation)
// }

// fn check_equation_valid_concat(equation: &Vec<usize>) -> bool {
//     if equation.len() == 2 {
//         return equation[1] == equation[0];
//     }
//     if equation[1] > equation[0] {
//         return false;
//     }

//     let mut add_equation = equation.clone();
//     let mut mul_equation = equation.clone();
//     let mut concat_equation = equation.clone();

//     add_equation.remove(2);
//     add_equation.remove(1);
//     add_equation.insert(1, equation[1] + equation[2]);

//     mul_equation.remove(2);
//     mul_equation.remove(1);
//     mul_equation.insert(1, equation[1] * equation[2]);

//     concat_equation.remove(2);
//     concat_equation.remove(1);
//     concat_equation.insert(1, equation[1] * 10usize.pow(equation[2].ilog10() + 1) + equation[2]);

//     check_equation_valid_concat(&add_equation) || check_equation_valid_concat(&mul_equation) || check_equation_valid_concat(&concat_equation)
// }

fn check_equation_valid(target: usize, equation: &Vec<usize>, index: usize, part2: bool) -> bool {
    if index == 0 {
        return target == equation[0]
    }

    let val = equation[index];
    let mut add = false;
    let mut mul = false;
    let mut concat = false;
    if target >= val {
        add = check_equation_valid(target - val, equation, index - 1, part2);
    }
    if target % val == 0 {
        mul = check_equation_valid(target / val, equation, index - 1, part2);
    }
    if part2 && target >= val{
        let ndigits = val.checked_ilog10().unwrap_or(0) + 1;
        let disconcat: usize = (target - val) / (10usize.pow(val.ilog10() + 1));
        if target % 10usize.pow(ndigits as u32) == val {
            concat = check_equation_valid(disconcat, equation, index - 1, part2);
        }
    }
    add || mul || concat
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Equation> {
    reader
        .lines()
        .flatten()
        .map(|line| {
            let (target, rest) = line.split_once(':').unwrap();
            Equation(
                target.parse().unwrap(),
                rest
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            )
        })
        .collect()
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let equations: Vec<Equation> = parse_input(reader);
        
        Ok(
            equations
                .into_iter()
                .filter(|equation| check_equation_valid(equation.0, &equation.1, equation.1.len() - 1, false))
                .map(|equation| equation.0)
                .sum()
        )
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let equations = parse_input(reader);

        Ok(
            equations
                .into_iter()
                .filter(|equation| check_equation_valid(equation.0, &equation.1, equation.1.len() - 1, true))
                .map(|equation| equation.0)
                .sum()
        )
    }
    
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
