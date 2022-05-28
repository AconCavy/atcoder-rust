#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use rand::prelude::StdRng;
use rand::Rng;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::fmt::{write, Display, Formatter};
use std::io;
use std::mem::*;
use std::time::{Duration, Instant};

#[fastout]
pub fn main() {
    input! {
        n: usize,
        t: usize,
        tiles: [Bytes; n],
    }

    const TIME_LIMIT: u64 = 2950;
    let start_temp: f64 = n as f64;
    let end_temp: f64 = (n * n) as f64;

    let simulator = AnnealingSimulator::new(TIME_LIMIT, start_temp, end_temp);
    let input = Input::new(n, t, tiles);
    let output = Output::new();
    let state = State::new(&input, output, input.tiles.clone());
    let result = simulator.simulate(state, 0);

    println!("{}", result.output);
}

pub fn pos_from_dir(origin: (usize, usize), dir: Direction) -> (usize, usize) {
    let delta: (i32, i32) = dir.into();
    let h = usize::try_from(origin.0 as i32 + delta.0).unwrap();
    let w = usize::try_from(origin.1 as i32 + delta.1).unwrap();
    (h, w)
}

pub fn dir_from_pos(origin: (usize, usize), pos: (usize, usize)) -> Direction {
    let dh = pos.0 as i32 - origin.0 as i32;
    let dw = pos.1 as i32 - origin.1 as i32;
    Direction::from((dh, dw))
}

#[derive(Debug, Clone)]
pub struct State<'a> {
    input: &'a Input,
    output: Output,
    tiles: Vec<Vec<u8>>,
}

impl<'a> State<'a> {
    pub fn new(input: &'a Input, output: Output, tiles: Vec<Vec<u8>>) -> Self {
        Self {
            input,
            output,
            tiles,
        }
    }
}

impl<'a> AnnealingState for State<'a> {
    fn calc_score(&self) -> f64 {
        let n = self.input.n;
        let mut dsu = Dsu::new(n * n);
        let mut tree = vec![true; n * n];
        for ch in 0..n {
            for cw in 0..n {
                if ch + 1 < n && self.tiles[ch][cw] & 8 != 0 && self.tiles[ch + 1][cw] & 2 != 0 {
                    let a = dsu.leader_of(ch * n + cw);
                    let b = dsu.leader_of((ch + 1) * n + cw);
                    if a == b {
                        tree[a] = false;
                    } else {
                        let t = tree[a] && tree[b];
                        dsu.merge(a, b);
                        tree[dsu.leader_of(a)] = t;
                    }
                }
                if cw + 1 < n && self.tiles[ch][cw] & 4 != 0 && self.tiles[ch][cw + 1] & 1 != 0 {
                    let a = dsu.leader_of(ch * n + cw);
                    let b = dsu.leader_of(ch * n + cw + 1);
                    if a == b {
                        tree[a] = false;
                    } else {
                        let t = tree[a] && tree[b];
                        dsu.merge(a, b);
                        tree[dsu.leader_of(a)] = t;
                    }
                }
            }
        }
        let mut trees = Vec::with_capacity(n * n);
        for ch in 0..n {
            for cw in 0..n {
                let u = ch * n + cw;
                let size = dsu.size_of(u);
                if self.tiles[ch][cw] != 0 && tree[dsu.leader_of(u)] && size > 1 {
                    trees.push(size);
                }
            }
        }

        trees.into_iter().fold(0.0, |acc, x| acc + x as f64)
    }

    fn arrange(&self, rng: &mut StdRng) -> Self {
        let mut output = self.output.clone();
        let mut tiles = self.tiles.clone();
        let n = self.input.n;
        for ch in 0..n {
            for cw in 0..n {
                if tiles[ch][cw] == 0 {
                    let d4 = DeltaIndex::new((n, n), &[(0, -1), (-1, 0), (0, 1), (1, 0)]);
                    let next = d4.generate((ch, cw)).collect::<Vec<_>>();
                    let (nh, nw) = next[rng.gen_range(0, next.len())];
                    let dir = dir_from_pos((ch, cw), (nh, nw));
                    output.apply(dir);

                    let tmp = tiles[ch][cw];
                    tiles[ch][cw] = tiles[nh][nw];
                    tiles[nh][nw] = tmp;

                    return Self {
                        input: self.input,
                        output,
                        tiles,
                    };
                }
            }
        }

        Self {
            input: self.input,
            output,
            tiles,
        }
    }

    fn is_valid(&self) -> bool {
        let len = self.output.directions.len();
        let mut ok = len <= self.input.t;
        ok &= !(len >= 4
            && self.output.directions[len - 1] == self.output.directions[len - 3]
            && self.output.directions[len - 2] == self.output.directions[len - 4]
            && self.output.directions[len - 1] != self.output.directions[len - 2]);
        ok
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    n: usize,
    t: usize,
    tiles: Vec<Vec<u8>>,
}

impl Input {
    fn new(n: usize, t: usize, tiles: Vec<Vec<u8>>) -> Self {
        let parse = |x: u8| match x {
            b'0'..=b'9' => Some(x - b'0'),
            b'a'..=b'f' => Some(x - b'a' + 10),
            _ => None,
        };

        let tiles = tiles
            .into_iter()
            .map(|ts| ts.into_iter().map(|x| parse(x).unwrap()).collect())
            .collect();
        Self { n, t, tiles }
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    directions: Vec<Direction>,
}

impl Output {
    fn new() -> Self {
        Self {
            directions: Vec::new(),
        }
    }

    fn apply(&mut self, dir: Direction) {
        self.directions.push(dir);
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.directions.iter().join(""))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    None,
    L,
    U,
    R,
    D,
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::None => '.',
            Direction::L => 'L',
            Direction::U => 'U',
            Direction::R => 'R',
            Direction::D => 'D',
        }
    }
}

impl Into<(i32, i32)> for Direction {
    fn into(self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::L => (0, -1),
            Direction::U => (-1, 0),
            Direction::R => (0, 1),
            Direction::D => (1, 0),
        }
    }
}

impl From<(i32, i32)> for Direction {
    fn from((h, w): (i32, i32)) -> Self {
        let h = h.min(1).max(-1);
        let w = w.min(1).max(-1);
        match (h, w) {
            (0, -1) => Direction::L,
            (-1, 0) => Direction::U,
            (0, 1) => Direction::R,
            (1, 0) => Direction::D,
            _ => Direction::None,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c: char = self.clone().into();
        write!(f, "{}", c)
    }
}

pub trait AnnealingState {
    fn calc_score(&self) -> f64;
    fn arrange(&self, rng: &mut StdRng) -> Self;
    fn is_valid(&self) -> bool;
}

pub struct AnnealingSimulator {
    time_limit_millis: u64,
    start_temp: f64,
    end_temp: f64,
}

impl AnnealingSimulator {
    pub fn new(time_limit_millis: u64, start_temp: f64, end_temp: f64) -> Self {
        Self {
            time_limit_millis,
            start_temp,
            end_temp,
        }
    }

    pub fn simulate<S: AnnealingState + Clone>(&self, init_state: S, random_seed: u64) -> S {
        let timer = Instant::now();
        let end = Duration::from_millis(self.time_limit_millis);
        let mut curr_state = init_state;
        let mut curr_score = curr_state.calc_score();
        let mut best_state = curr_state.clone();
        let mut best_score = curr_score;
        let mut rng_prob: StdRng = rand::SeedableRng::seed_from_u64(random_seed);
        let mut rng_modify: StdRng = rand::SeedableRng::seed_from_u64(random_seed);

        let calc_temp = |elapsed: f64| -> f64 {
            self.start_temp
                + (self.end_temp - self.start_temp) * elapsed / self.time_limit_millis as f64
        };

        let calc_prob = |score_from: f64, score_to: f64, temp: f64| -> f64 {
            let delta = (score_to - score_from).min(0.0) as f64;
            (delta / temp).exp()
        };

        while timer.elapsed() < end {
            let next_state = curr_state.arrange(&mut rng_modify);
            if next_state.is_valid() {
                let next_score = next_state.calc_score();
                let temp = calc_temp(timer.elapsed().as_millis() as f64);
                if calc_prob(curr_score, next_score, temp) >= rng_prob.gen::<f64>() {
                    curr_state = next_state;
                    curr_score = next_score;

                    if curr_score >= best_score {
                        best_state = curr_state.clone();
                        best_score = curr_score;
                    }
                }
            }
        }

        best_state
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

pub struct DeltaIndex<'a> {
    limit: (usize, usize),
    delta: &'a [(i32, i32)],
}

impl<'a> DeltaIndex<'a> {
    pub fn new(limit: (usize, usize), delta: &'a [(i32, i32)]) -> Self {
        Self { limit, delta }
    }

    pub fn generate(&self, origin: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || {
            while i < self.delta.len() {
                let dest = (
                    usize::try_from(origin.0 as i32 + self.delta[i].0).unwrap_or(self.limit.0),
                    usize::try_from(origin.1 as i32 + self.delta[i].1).unwrap_or(self.limit.1),
                );
                i += 1;
                if dest.0 < self.limit.0 && dest.1 < self.limit.1 {
                    return Some(dest);
                }
            }
            None
        })
    }
}
