use anyhow::*;
use grid::{Dir, Grid, Vector};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use core::fmt::Display;

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    START,
    END,
    WALL,
    EMPTY,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::START,
            'E' => Self::END,
            '#' => Self::WALL,
            _ => Self::EMPTY,
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::START => 'S',
            Tile::END => 'E',
            Tile::WALL => '#',
            Tile::EMPTY => '.',
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct ReindeerState {
    pos: Vector,
    dir: Dir,
    cost: usize,
    path: Vec<Vector>,
}

impl ReindeerState {
    fn new(pos: Vector, dir: Dir, cost: usize, path: Vec<Vector>) -> Self {
        Self {
            pos,
            dir,
            cost,
            path,
        }
    }
}

impl Ord for ReindeerState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for ReindeerState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
} 

impl ReindeerState {
    fn rotate_dir(&self) -> (Dir, Dir) {
        if self.dir == Dir::LEFT || self.dir == Dir::RIGHT {
            (Dir::UP, Dir::DOWN)
        } else {
            (Dir::LEFT, Dir::RIGHT)
        }
    }
}

struct Maze {
    grid: Vec<Vec<Tile>>,
    start: Vector,
    width: usize,
    height: usize,
}

impl Maze {
    fn shortest_path(&self) -> (usize, Vec<Vec<Vector>>) {
        let mut score  = usize::MAX;
        let mut best_spots = Vec::new();

        let mut visited = FxHashMap::default();
        let mut frontier = BinaryHeap::new();
        frontier.push(ReindeerState::new(self.start, Dir::RIGHT, 0, vec![self.start]));

        while let Some(state) = frontier.pop() {
            let cost = state.cost;
            if let Some(&prev_cost) = visited.get(&(state.pos, state.dir)) {
                if state.cost > prev_cost {
                    continue;
                }
            } else {
                visited.insert((state.pos, state.dir), state.cost);
            }

            if self.has_reached_finish(&state) && cost <= score {
                score = cost;
                best_spots.push(state.path.clone());
            }

            for new_state in self.get_neighbors(&state) {
                frontier.push(new_state);
            }
        }

        println!("{}", best_spots.len());

        (score, best_spots)
    }

    fn get_neighbors(&self, state: &ReindeerState) -> Vec<ReindeerState> {
        let mut neighbors = Vec::new();
        if self.can_move(state.pos, state.dir) {
            neighbors.push(ReindeerState::new(state.pos + state.dir.into(), state.dir, state.cost + 1, {
                let mut path = state.path.clone();
                path.push(state.pos + state.dir.into());
                path
            }));
        }
        let neighbor_dir = state.rotate_dir();
        if self.can_move(state.pos, neighbor_dir.0) {
            neighbors.push(ReindeerState::new(state.pos + neighbor_dir.0.into(), neighbor_dir.0, state.cost + 1001, {
                let mut path = state.path.clone();
                path.push(state.pos + neighbor_dir.0.into());
                path
            }));
        }

        if self.can_move(state.pos, neighbor_dir.1) {
            neighbors.push(ReindeerState::new(state.pos + neighbor_dir.1.into(), neighbor_dir.1, state.cost + 1001, {
                let mut path = state.path.clone();
                path.push(state.pos + neighbor_dir.1.into());
                path
            }));
        }

        neighbors
    }

    fn has_reached_finish(&self, state: &ReindeerState) -> bool {
        let pos = state.pos;
        self.grid[pos.0 as usize][pos.1 as usize] == Tile::END
    }
}

impl Grid for Maze {
    fn new_from_map(map: &str) -> Self {
        let mut grid = Vec::new();
        let mut start_pos = Vector(0, 0);

        for (row, line) in map.lines().enumerate() {
            let mut grid_row = Vec::new();

            for (col, char) in line.char_indices() {
                let tile = Tile::from(char);

                if tile == Tile::START {
                    start_pos = Vector(row as i32, col as i32);
                }

                grid_row.push(tile);
            }
            grid.push(grid_row);
        }

        let height = grid.len();
        let width = grid[0].len();

        Maze {
            grid,
            start: start_pos,
            width,
            height,
        }
    }

    fn can_move(&self, pos: Vector, dir: Dir) -> bool {
        let new_pos = pos + dir.into();
        if !self.is_in_map(new_pos) {
            return false;
        }
        self.grid[new_pos.0 as usize][new_pos.1 as usize] != Tile::WALL
    }

    fn is_in_map(&self, pos: Vector) -> bool {
       pos.0 >= 0 && pos.0 < self.height as i32 && pos.1 >= 0 && pos.1 < self.width as i32
    }

}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .grid
            .iter()
            .map(|row| row.iter().map(|&tile| char::from(tile)).collect::<String>())
            .join("\n");

        writeln!(f, "{string}")
    }
}

fn parse_input<R: BufRead>(reader: R) -> Maze {
    let input = reader
        .lines()
        .flatten()
        .join("\n");

    Maze::new_from_map(&input)
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let maze = parse_input(reader);
        Ok(maze.shortest_path().0)
    }

    assert_eq!(7036, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let maze = parse_input(reader);
        Ok(maze.shortest_path().1.iter().flatten().unique().count())
    }
    
    assert_eq!(45, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
