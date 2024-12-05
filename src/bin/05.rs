#![feature(is_sorted)]

use anyhow::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rule(usize, usize);

fn parse_rules(input: &Vec<String>) -> HashSet<Rule> {
    let rule_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let rules = input
        .iter()
        .filter(|line| rule_regex.is_match(line))
        .map(|line| {
            let captures = rule_regex.captures(&line).unwrap();
            (captures.get(1).unwrap().as_str().parse::<usize>().unwrap(), captures.get(2).unwrap().as_str().parse::<usize>().unwrap())
        })
        .map(|(i, j)| Rule(i, j))
        .collect();

    rules
}

fn parse_updates(input: &Vec<String>) -> Vec<Vec<usize>> {
    let rule_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let updates: Vec<Vec<_>> = input
        .iter()
        .filter(|line| !rule_regex.is_match(line))
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    updates
}

fn check_update(update: &Vec<usize>, rules: &HashSet<Rule>) -> bool {
    update.is_sorted_by(|a, b| !rules.contains(&Rule(*b, *a)))
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Vec<String> = reader
            .lines()
            .flatten()
            .collect();

        let rules = parse_rules(&input);
        let updates = parse_updates(&input);
        let valid: Vec<Vec<usize>> = updates
            .iter()
            .cloned()
            .filter(|update| check_update(update, &rules))
            .collect();

        Ok(
            valid
                .into_iter()
                .map(|update| update[update.len() / 2])
                .sum()
        )
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input: Vec<String> = reader
            .lines()
            .flatten()
            .collect();

        let rules = parse_rules(&input);
        let updates = parse_updates(&input);

        let invalid: Vec<Vec<usize>> = updates
            .iter()
            .cloned()
            .filter(|update| !check_update(update, &rules))
            .collect();

        let fixed: Vec<Vec<usize>> = invalid
            .into_iter()
            .map(|mut update| {
                update.sort_by(|a, b| {
                    if rules.contains(&Rule(*a, *b)) {
                        Ordering::Less
                    } else if rules.contains(&Rule(*b, *a)) {
                        Ordering::Greater
                    } else {
                        unreachable!()
                    }
                });
                update
            })
            .collect();

        Ok(
            fixed
                .into_iter()
                .map(|update| update[update.len() / 2])
                .sum()
        )
    }
    
    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
