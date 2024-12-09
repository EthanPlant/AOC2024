use anyhow::*;
use itertools::enumerate;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i32, i32);

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: FxHashMap<char, Vec<Pos>>,
}

impl Map {
    pub fn find_antinode(&self, antenna1: &Pos, antenna2: &Pos, part2: bool) -> Vec<Pos> {
        let dist = *antenna2 - *antenna1;
        let mut antinodes = vec![*antenna2 + dist, *antenna1 - dist];

        if part2 {
            let mut pos1 = *antenna1 - dist;
            let mut pos2 = *antenna2 + dist;

            while self.is_in_map(&pos1) {
                antinodes.push(pos1);
                pos1 -= dist;
            }

            while self.is_in_map(&pos2) {
                antinodes.push(pos2);
                pos2 += dist;

            }
        }

        antinodes
    }

    pub fn is_in_map(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.0 < self.width as i32 && pos.1 >= 0 && pos.1 < self.height as i32
    }
}

fn parse_input<R: BufRead>(reader: R) -> Map {
    let input: Vec<Vec<char>> = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();

    let width = input[0].len();
    let height = input.len();
    let mut antennas: FxHashMap<char, Vec<Pos>> = FxHashMap::default();

    for i in 0..width {
        for j in 0..height {
            let c = input[j][i];
            if c != '.' {
                match antennas.get_mut(&c) {
                    Some(antennas) => antennas.push(Pos(i as i32, height as i32 - j as i32 - 1)),
                    None => {
                        antennas.insert(c, vec![Pos(i as i32, height as i32 - j as i32 - 1)]);
                    }
                }
            }
        }
    }

    Map {
        width,
        height,
        antennas
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_input(reader);

        let mut antinodes = FxHashSet::default();

        for antennas in map.antennas.values() {
            for (i, a1) in enumerate(antennas) {
                for a2 in &antennas[i + 1..] {
                    let anti = map.find_antinode(a1, a2, false);
                    antinodes.extend(anti);
                }
            }
        }

        Ok(
            antinodes
                .into_iter()
                .filter(|antinode| map.is_in_map(antinode))
                .count()
        )
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_input(reader);

        let mut antinodes = FxHashSet::default();

        for antennas in map.antennas.values() {
            if antennas.len() > 1 {
                antinodes.extend(antennas.clone());
            }

            for (i, a1) in enumerate(antennas) {
                for a2 in &antennas[i + 1..] {
                    let anti = map.find_antinode(a1, a2, true);
                    antinodes.extend(anti);
                }
            }
        }

        Ok(
            antinodes
                .into_iter()
                .filter(|antinode| map.is_in_map(antinode))
                .count()
        )
    }
    
    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
