use anyhow::*;
use grid::{Dir, Grid, Vector};
use itertools::Itertools;
use std::collections::VecDeque;
use core::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "15"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}

impl Tile {
    fn double(self) -> impl Iterator<Item = Self> {
        match self {
            Tile::Empty => [Tile::Empty, Tile::Empty].into_iter(),
            Tile::Wall => [Tile::Wall, Tile::Wall].into_iter(),
            Tile::Box => [Tile::BoxLeft, Self::BoxRight].into_iter(),
            Tile::Robot => [Tile::Robot, Tile::Empty].into_iter(),
            _ => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => Self::Empty,
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Robot => '@',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
        }
    }
}

struct Robot {
    pos: Vector,
    movement: VecDeque<Dir>,
}

struct Warehouse {
    grid: Vec<Vec<Tile>>,
    robot: Robot,
    width: usize,
    height: usize,
}

impl Warehouse {
    fn new_from_map(map: &str, movement: VecDeque<Dir>, part2: bool) -> Self {
        let mut grid: Vec<Vec<Tile>> = Vec::new();
        let mut robot_pos = Vector(0, 0);

        for (row, line) in map.lines().enumerate() {
            let mut grid_row = Vec::new();

            for (col, char) in line.char_indices() {
                let tile = Tile::from(char);

                if tile == Tile::Robot {
                    robot_pos = Vector(row as i32, col as i32);
                }

                grid_row.push(tile);
            }
            grid.push(grid_row);
        }

        if part2 {
            grid = Self::scale_width(grid);
        }

        let height = grid.len();
        let width = grid[0].len();

        if part2 {
            robot_pos.1 *= 2;
        }

        Self {
            grid,
            robot: Robot {
                pos: robot_pos,
                movement,
            },
            width,
            height,
        } 
    }

    fn scale_width(grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        grid
            .into_iter()
            .map(|row| row.into_iter().flat_map(Tile::double).collect())
            .collect()
    }

    fn move_robot(&mut self) {
        let dir = self.robot.movement.pop_front().unwrap();

        if self.can_move(self.robot.pos, dir) {
            self.move_tile(self.robot.pos, dir);
            self.robot.pos += dir.into();
        }
    }

    fn move_tile(&mut self, pos: Vector, dir: Dir) {
        let next_pos = pos + dir.into();
        let next_tile = self.grid[next_pos.0 as usize][next_pos.1 as usize];

        match next_tile {
            Tile::Empty => {
                self.grid[next_pos.0 as usize][next_pos.1 as usize] = self.grid[pos.0 as usize][pos.1 as usize];
                self.grid[pos.0 as usize][pos.1 as usize] = Tile::Empty;
            },
            Tile::Box => {
                self.move_tile(next_pos, dir);
                self.grid[next_pos.0 as usize][next_pos.1 as usize] = self.grid[pos.0 as usize][pos.1 as usize];
                self.grid[pos.0 as usize][pos.1 as usize] = Tile::Empty;
            },
            Tile::BoxLeft => {
                self.move_tile(Vector(next_pos.0, next_pos.1 + 1), dir);
                self.move_tile(next_pos, dir);
                self.grid[next_pos.0 as usize][next_pos.1 as usize] = self.grid[pos.0 as usize][pos.1 as usize];
                self.grid[pos.0 as usize][pos.1 as usize] = Tile::Empty;
            },
            Tile::BoxRight => {
                self.move_tile(Vector(next_pos.0, next_pos.1 - 1), dir);
                self.move_tile(next_pos, dir);
                self.grid[next_pos.0 as usize][next_pos.1 as usize] = self.grid[pos.0 as usize][pos.1 as usize];
                self.grid[pos.0 as usize][pos.1 as usize] = Tile::Empty;
            },
            Tile::Wall => unreachable!(),
            Tile::Robot => unreachable!(),
        }
    }

    fn gps_coordinate(pos: Vector) -> usize {
        pos.0 as usize * 100 + pos.1 as usize
    }

    fn sum_gps(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.grid[row][col] == Tile::Box || self.grid[row][col] == Tile::BoxLeft {
                    sum += Self::gps_coordinate(Vector(row as i32, col as i32));
                }
            }
        }

        sum
    }
}

impl Grid for Warehouse {
    fn can_move(&self, pos: Vector, dir: Dir) -> bool {
        let next_pos = pos + dir.into();
        let next_tile = self.grid[next_pos.0 as usize][next_pos.1 as usize];

        match next_tile {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Box => self.can_move(next_pos, dir),
            Tile::BoxLeft => {
                if dir == Dir::LEFT {
                    self.can_move(next_pos, dir)
                } else if dir == Dir::RIGHT {
                    self.can_move(Vector(next_pos.0, next_pos.1 + 1), dir)
                } else {
                    self.can_move(Vector(next_pos.0, next_pos.1 + 1), dir) &&
                        self.can_move(next_pos, dir)
                }
            },
            Tile::BoxRight =>{
                if dir == Dir::RIGHT {
                    self.can_move(next_pos, dir)
                } else if dir == Dir::LEFT {
                    self.can_move(Vector(next_pos.0, next_pos.1 - 1), dir)
                } else {
                    self.can_move(Vector(next_pos.0, next_pos.1 - 1), dir) &&
                        self.can_move(next_pos, dir)
                }
            },
            Tile::Robot => unreachable!(),
        }
    }

    fn new_from_map(_: &str) -> Self {
        todo!()
    }

    fn is_in_map(&self, _: Vector) -> bool {
        todo!()
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .grid
            .iter()
            .map(|row| row.iter().map(|&tile| char::from(tile)).collect::<String>())
            .join("\n");

        writeln!(f, "{string}")
    }
}

fn parse_input<R: BufRead>(reader: R, part2: bool) -> Warehouse {
    let input = reader
        .lines()
        .flatten()
        .join("\n");

    let (map, movement) = input.split_once("\n\n").unwrap();
    let moves: VecDeque<Dir> = movement
        .lines()
        .flat_map(|line| line.chars())
        .map(Dir::from)
        .collect();

    Warehouse::new_from_map(map, moves, part2)
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut warehouse = parse_input(reader, false);

        while !warehouse.robot.movement.is_empty() {
            warehouse.move_robot();
        }
        Ok(warehouse.sum_gps())
    }

    assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut warehouse = parse_input(reader, true);

        while !warehouse.robot.movement.is_empty() {
            warehouse.move_robot();
        }
        Ok(warehouse.sum_gps())
    }
    
    assert_eq!(9021, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
