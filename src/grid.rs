use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector(pub i32, pub i32);

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Dir{
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::UP, Self::LEFT, Self::DOWN, Self::RIGHT].into_iter()
    }
}

impl From<Dir> for Vector {
    fn from(value: Dir) -> Self {
        match value {
            Dir::UP => Vector(-1, 0),
            Dir::DOWN => Vector(1, 0),
            Dir::LEFT => Vector(0, -1),
            Dir::RIGHT => Vector(0, 1),
        }
    }
}

impl From<Vector> for Dir {
    fn from(value: Vector) -> Self {
        match value {
            Vector(-1, 0) => Dir::UP,
            Vector(1, 0) => Dir::DOWN,
            Vector(0, -1) => Dir::LEFT,
            Vector(0, 1) => Dir::RIGHT,
            _ => unimplemented!(),
        }
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '>' => Dir::RIGHT,
            '^' => Dir::UP,
            '<' => Dir::LEFT,
            'v' => Dir::DOWN,
            _ => unreachable!()
        }
    }
}

pub trait Grid {
    fn new_from_map(map: &str) -> Self;
    fn can_move(&self, pos: Vector, dir: Dir) -> bool;
    fn is_in_map(&self, vec: Vector) -> bool;
}