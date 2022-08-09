#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use rand::rngs::StdRng;
use rand::Rng;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::fmt::{write, Display, Formatter};
use std::io;
use std::mem::*;
use std::time::{Duration, Instant};

#[fastout]
fn main() {
    let input = Input::scan();
    let mut solver = Solver::new(&input, 0);
    let output = solver.solve();
    println!("{}", output);
}

pub struct Solver {
    n: usize,
    k: usize,
    g: Vec<Vec<u8>>,
    action_limit: usize,
    rng: StdRng,
}

impl Solver {
    const USED: u8 = 255;

    pub fn new(input: &Input, seed: u64) -> Self {
        Self {
            n: input.n,
            k: input.k,
            g: input.g.clone(),
            action_limit: input.k * 100,
            rng: rand::SeedableRng::seed_from_u64(seed),
        }
    }

    pub fn solve(&mut self) -> Output {
        Output {
            arranges: self.arrange(self.action_limit / 2),
            connects: self.connect(),
        }
    }

    pub fn is_valid(&self, p: Point, dir: Direction) -> Option<Point> {
        let delta = Vector::from(dir);
        let row = usize::try_from(p.row as i32 + delta.row).unwrap_or(self.n);
        let col = usize::try_from(p.col as i32 + delta.col).unwrap_or(self.n);
        if row < self.n && col < self.n {
            Some(Point { row, col })
        } else {
            None
        }
    }

    pub fn can_arrange(&self, p: Point, dir: Direction) -> Option<Point> {
        if let Some(np) = self.is_valid(p, dir) {
            if self.g[np.row][np.col] == 0 {
                return Some(np);
            }
        }

        None
    }

    pub fn can_connect(&self, p: Point, dir: Direction) -> Option<Point> {
        let mut np = p;
        while let Some(tmp) = self.is_valid(np, dir) {
            if self.g[tmp.row][tmp.col] == self.g[p.row][p.col] {
                return Some(tmp);
            } else if self.g[tmp.row][tmp.col] != 0 {
                return None;
            }
            np = tmp;
        }

        None
    }

    pub fn arrange(&mut self, arrange_limit: usize) -> Vec<(Point, Point)> {
        let mut result = Vec::new();
        for _ in 0..arrange_limit {
            let cp = Point {
                row: self.rng.gen_range(0, self.n),
                col: self.rng.gen_range(0, self.n),
            };
            let dir = Solver::idx_to_dir(self.rng.gen_range(0, 4));

            if self.g[cp.row][cp.col] == 0 {
                continue;
            }

            if let Some(np) = self.can_arrange(cp, dir) {
                let tmp = self.g[np.row][np.col];
                self.g[np.row][np.col] = self.g[cp.row][cp.col];
                self.g[cp.row][cp.col] = tmp;

                result.push((cp, np));
                self.action_limit -= 1;
            }
        }

        result
    }

    pub fn connect(&mut self) -> Vec<(Point, Point)> {
        let mut result = Vec::new();

        for row in 0..self.n {
            for col in 0..self.n {
                if self.g[row][col] != 0 && self.g[row][col] != Solver::USED {
                    for dir in (0..2).map(Solver::idx_to_dir) {
                        let cp = Point { row, col };
                        if self.can_connect(cp, dir).is_some() {
                            let mut np = cp;
                            while let Some(tmp) = self.is_valid(np, dir) {
                                if self.g[tmp.row][tmp.col] == self.g[cp.row][cp.col] {
                                    result.push((cp, tmp));
                                    self.action_limit -= 1;
                                    break;
                                }
                                self.g[tmp.row][tmp.col] = Solver::USED;
                                np = tmp;
                            }

                            if self.action_limit == 0 {
                                return result;
                            }
                        }
                    }
                }
            }
        }

        result
    }

    fn idx_to_dir(idx: usize) -> Direction {
        match idx {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => Direction::Left,
        }
    }
}

pub struct Input {
    n: usize,
    k: usize,
    g: Vec<Vec<u8>>,
}

impl Input {
    pub fn scan() -> Self {
        input! {
            n: usize,
            k: usize,
            g: [Bytes; n],
        }
        Self {
            n,
            k,
            g: g.into_iter()
                .map(|x| x.into_iter().map(|v| v - b'0').collect())
                .collect(),
        }
    }
}

pub struct Output {
    arranges: Vec<(Point, Point)>,
    connects: Vec<(Point, Point)>,
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.arranges.len())?;
        for (p0, p1) in &self.arranges {
            writeln!(f, "{} {}", p0, p1)?;
        }
        writeln!(f, "{}", self.connects.len())?;
        for (p0, p1) in &self.connects {
            writeln!(f, "{} {}", p0, p1)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.row, self.col)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vector {
    row: i32,
    col: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl From<Direction> for Vector {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Right => Vector { row: 0, col: 1 },
            Direction::Down => Vector { row: 1, col: 0 },
            Direction::Left => Vector { row: 0, col: -1 },
            Direction::Up => Vector { row: -1, col: 0 },
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dir = match self {
            Direction::Right => "Right",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Up => "Up",
        };
        write!(f, "{}", dir)
    }
}

pub struct Dsu {
    len: usize,
    parent_or_size: Vec<i32>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            len: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn merge(&mut self, u: usize, v: usize) -> usize {
        assert!(u < self.len);
        assert!(v < self.len);
        let (mut x, mut y) = (self.leader_of(u), self.leader_of(v));
        if x == y {
            return x;
        }

        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            swap(&mut x, &mut y);
        }

        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        assert!(u < self.len);
        assert!(v < self.len);
        self.leader_of(u) == self.leader_of(v)
    }

    pub fn leader_of(&mut self, u: usize) -> usize {
        assert!(u < self.len);
        if self.parent_or_size[u] < 0 {
            return u;
        }

        self.parent_or_size[u] = self.leader_of(self.parent_or_size[u] as usize) as i32;
        self.parent_or_size[u] as usize
    }

    pub fn size_of(&mut self, u: usize) -> usize {
        assert!(u < self.len);
        let u = self.leader_of(u);
        -self.parent_or_size[u] as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_of = vec![0; self.len];
        let mut group_size = vec![0; self.len];
        for i in 0..self.len {
            leader_of[i] = self.leader_of(i);
            group_size[leader_of[i]] += 1;
        }
        let mut result: Vec<Vec<_>> = group_size.into_iter().map(Vec::with_capacity).collect();
        for (i, v) in leader_of.into_iter().enumerate() {
            result[v].push(i);
        }
        result.into_iter().filter(|x| !x.is_empty()).collect()
    }
}
