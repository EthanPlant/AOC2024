use anyhow::*;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const DIR: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 0),
];

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_neighbors(pos: (i32, i32), map: &Vec<Vec<char>>, plant: char) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();

    for dir in DIR {
        let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if (neighbor_pos.0 as usize) < map.len() && (neighbor_pos.1 as usize) < map[0].len() && map[neighbor_pos.0 as usize][neighbor_pos.1 as usize] == plant {
            neighbors.push(neighbor_pos);
        }
    }

    neighbors
}

fn bfs(pos: (i32, i32), visited: &mut FxHashSet<(i32, i32)>, map: &Vec<Vec<char>>) -> (FxHashSet<(i32, i32)>, usize) {
    let mut queue = VecDeque::new();
    let plant = map[pos.0 as usize][pos.1 as usize];
    let mut area = FxHashSet::default();
    let mut perimiter = 0;
    visited.insert(pos);
    queue.push_back(pos);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        area.insert(curr);
        let neighbors = get_neighbors(curr, map, plant);
        perimiter += 4 - neighbors.len();
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    (area, perimiter)
}

fn count_region_sides(region: &FxHashSet<(i32, i32)>) -> usize {
    let mut side_count = 0;
    for dir in DIR {
        let mut sides = FxHashSet::default();
        for pos in region {
            let tmp = (pos.0 + dir.0, pos.1 + dir.1);
            if !region.contains(&tmp) {
                sides.insert(tmp);
            }
        }
        let mut remove = FxHashSet::default();
        for side in &sides {
            let mut tmp = (side.0 + dir.1, side.1 + dir.0);
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = parse_input(reader);
        let mut visited = FxHashSet::default();
        let mut price = 0;
        
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if !visited.contains(&(i as i32, j as i32)) {
                    let (region_area, region_perim) = bfs((i as i32, j as i32), &mut visited, &input);
                    price += region_area.len() * region_perim;
                }
            }
        }

        Ok(price)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = parse_input(reader);
        let mut visited = FxHashSet::default();
        let mut price = 0;
        
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if !visited.contains(&(i as i32, j as i32)) {
                    let (region_area, _) = bfs((i as i32, j as i32), &mut visited, &input);
                    let sides = count_region_sides(&region_area);
                    price += region_area.len() * sides;
                }
            }
        }

        Ok(price)
    }
    
    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
