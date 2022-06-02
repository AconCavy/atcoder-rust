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

    let mut input = Input::new(n, t, tiles);

    let mut result = Vec::new();
    let mut pos_0 = search_pos_0(n, &input.tiles);
    let mut used = vec![vec![0; n]; n];
    let mut tiles_decided = vec![vec![0; n]; n];

    for k in 0..n - 1 {
        for w1 in (k..n - 1).rev().merge(vec![n - 1]) {
            let h1 = k;
            let t1 = get_suitable_tile(n, &tiles_decided, (h1, w1));
            if let Some((h2, w2)) = search_pos(n, &input.tiles, &used, &tiles_decided, (h1, w1), t1)
            {
                let t2 = input.tiles[h2][w2];
                let pos_to = if w1 + 1 < n {
                    (h1, w1 + 1)
                } else if h1 + 1 < n {
                    (h1 + 1, w1)
                } else {
                    (h1, w1)
                };
                pos_0 = tile_navigate_to(
                    n,
                    &mut input.tiles,
                    &mut used,
                    pos_0,
                    (h2, w2),
                    pos_to,
                    &mut result,
                );
                used[pos_to.0][pos_to.1] = 1;
                tiles_decided[h1][w1] = t2;
            }
        }

        while pos_0.1 > k {
            pos_0 = move_left(n, &mut input.tiles, pos_0, &mut result);
        }
        while pos_0.0 > k {
            pos_0 = move_up(n, &mut input.tiles, pos_0, &mut result);
        }
        while pos_0.1 + 1 < n {
            pos_0 = move_right(n, &mut input.tiles, pos_0, &mut result);
        }
        pos_0 = move_down(n, &mut input.tiles, pos_0, &mut result);

        for h in 0..n {
            for w in 0..n {
                used[h][w] = 1;
            }
        }

        for h in k + 1..n {
            for w in k..n {
                used[h][w] = 0;
            }
        }

        for h1 in (k + 1..n - 1).rev().merge(vec![n - 1]) {
            let w1 = k;
            let t1 = get_suitable_tile(n, &tiles_decided, (h1, w1));
            if let Some((h2, w2)) = search_pos(n, &input.tiles, &used, &tiles_decided, (h1, w1), t1)
            {
                let t2 = input.tiles[h2][w2];
                let pos_to = if h1 + 1 < n {
                    (h1 + 1, w1)
                } else if w1 + 1 < n {
                    (h1, w1 + 1)
                } else {
                    (h1, w1)
                };
                pos_0 = tile_navigate_to(
                    n,
                    &mut input.tiles,
                    &mut used,
                    pos_0,
                    (h2, w2),
                    pos_to,
                    &mut result,
                );
                used[pos_to.0][pos_to.1] = 1;
                tiles_decided[h1][w1] = t2;
            }
        }

        while pos_0.0 > k + 1 {
            pos_0 = move_up(n, &mut input.tiles, pos_0, &mut result);
        }
        while pos_0.1 > k {
            pos_0 = move_left(n, &mut input.tiles, pos_0, &mut result);
        }
        while pos_0.0 + 1 < n {
            pos_0 = move_down(n, &mut input.tiles, pos_0, &mut result);
        }
        pos_0 = move_right(n, &mut input.tiles, pos_0, &mut result);

        for h in 0..n {
            for w in 0..n {
                used[h][w] = 1;
            }
        }

        for h in k + 1..n {
            for w in k + 1..n {
                used[h][w] = 0;
            }
        }
    }

    println!("{}", result.iter().take(t).join(""));
}

fn get_suitable_tile(n: usize, tiles_decided: &[Vec<u8>], pos: (usize, usize)) -> u8 {
    let mut result = 0;
    if pos.0 > 0 && tiles_decided[pos.0 - 1][pos.1] & 8 != 0 {
        result |= 2;
    }

    if pos.1 > 0 && tiles_decided[pos.0][pos.1 - 1] & 4 != 0 {
        result |= 1;
    }

    if pos.0 + 1 < n && tiles_decided[pos.0 + 1][pos.1] & 2 != 0 {
        result |= 8;
    }

    if pos.1 + 1 < n && tiles_decided[pos.0][pos.1 + 1] & 1 != 0 {
        result |= 4;
    }

    if result == 0 {
        if pos.0 > 0 {
            result |= 2;
        }
        if pos.1 > 0 {
            result |= 1;
        }

        if pos.0 + 1 < n {
            result |= 8;
        }

        if pos.1 + 1 < n {
            result |= 4;
        }
    }

    result
}

fn search_pos_0(n: usize, tiles: &[Vec<u8>]) -> (usize, usize) {
    for i in 0..n {
        for j in 0..n {
            if tiles[i][j] == 0 {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn cycle_dsu(n: usize, tiles_decided: &[Vec<u8>]) -> Dsu {
    let mut dsu = Dsu::new(n * n);
    for h1 in 0..n {
        for w1 in 0..n {
            let h2 = h1 + 1;
            let w2 = w1;
            if h2 < n && tiles_decided[h1][w1] & 8 != 0 && tiles_decided[h2][w2] & 2 != 0 {
                dsu.merge(h1 * n + w1, h2 * n + w2);
            }

            let h2 = h1;
            let w2 = w1 + 1;
            if w2 < n && tiles_decided[h1][w1] & 4 != 0 && tiles_decided[h2][w2] & 1 != 0 {
                dsu.merge(h1 * n + w1, h2 * n + w2);
            }
        }
    }

    dsu
}

fn check_pos(
    n: usize,
    tiles: &[Vec<u8>],
    tiles_decided: &[Vec<u8>],
    pos_from: (usize, usize),
    pos_to: (usize, usize),
) -> (bool, usize) {
    let mut result = true;
    let mut dsu = cycle_dsu(n, tiles_decided);
    let mut neighbour = Vec::new();
    if tiles[pos_from.0][pos_from.1] & 1 != 0
        && pos_to.1 > 0
        && tiles_decided[pos_to.0][pos_to.1 - 1] & 4 != 0
    {
        neighbour.push(pos_to.0 * n + (pos_to.1 - 1));
    }
    if tiles[pos_from.0][pos_from.1] & 2 != 0
        && pos_to.0 > 0
        && tiles_decided[pos_to.0 - 1][pos_to.1] & 8 != 0
    {
        neighbour.push((pos_to.0 - 1) * n + pos_to.1);
    }
    if tiles[pos_from.0][pos_from.1] & 4 != 0
        && pos_to.1 + 1 < n
        && tiles_decided[pos_to.0][pos_to.1 + 1] & 1 != 0
    {
        neighbour.push(pos_to.0 * n + (pos_to.1 + 1));
    }
    if tiles[pos_from.0][pos_from.1] & 8 != 0
        && pos_to.0 + 1 < n
        && tiles_decided[pos_to.0 + 1][pos_to.1] & 2 != 0
    {
        neighbour.push((pos_to.0 + 1) * n + pos_to.1);
    }

    for i in 0..neighbour.len() {
        for j in i + 1..neighbour.len() {
            let u = dsu.leader_of(neighbour[i]);
            let v = dsu.leader_of(neighbour[j]);
            result &= u != v;
            dsu.merge(u, v);
        }
    }

    let max_tree = dsu.groups().iter().fold(0, |acc, x| max(acc, x.len()));
    (result, max_tree)
}

fn search_pos(
    n: usize,
    tiles: &[Vec<u8>],
    used: &[Vec<i32>],
    tiles_decided: &[Vec<u8>],
    pos: (usize, usize),
    t: u8,
) -> Option<(usize, usize)> {
    let mut list = Vec::new();
    for h in 0..n {
        for w in 0..n {
            if used[h][w] == 0 && tiles[h][w] & t == t {
                let (ok, size) = check_pos(n, tiles, tiles_decided, (h, w), pos);
                if ok {
                    list.push(((h, w), size));
                }
            }
        }
    }

    if !list.is_empty() {
        list.sort_by_key(|x| x.1);
        list.reverse();
        let (pos, _) = list[0];
        return Some(pos);
    }

    for h in 0..n {
        for w in 0..n {
            if used[h][w] == 0 && tiles[h][w] & t != 0 {
                let (ok, size) = check_pos(n, tiles, tiles_decided, (h, w), pos);
                if ok {
                    list.push(((h, w), size));
                }
            }
        }
    }

    if list.is_empty() {
        None
    } else {
        list.sort_by_key(|x| x.1);
        list.reverse();
        let (pos, _) = list[0];
        Some(pos)
    }
}

fn swap_tile(tiles: &mut [Vec<u8>], pos_1: (usize, usize), pos_2: (usize, usize)) {
    let tmp = tiles[pos_1.0][pos_1.1];
    tiles[pos_1.0][pos_1.1] = tiles[pos_2.0][pos_2.1];
    tiles[pos_2.0][pos_2.1] = tmp;
}

fn calc_prev(n: usize, used: &[Vec<i32>], pos: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let mut prev = vec![vec![(n, n); n]; n];
    let mut visit = vec![vec![false; n]; n];
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    visit[pos.0][pos.1] = true;
    let delta = DeltaIndex::new((n, n), &[(0, -1), (-1, 0), (0, 1), (1, 0)]);
    while let Some(curr) = queue.pop_front() {
        for next in delta.generate(curr) {
            if used[next.0][next.1] == 0 && !visit[next.0][next.1] {
                visit[next.0][next.1] = true;
                prev[next.0][next.1] = curr;
                queue.push_back(next);
            }
        }
    }

    prev
}

fn move_left(
    _n: usize,
    tiles: &mut [Vec<u8>],
    pos: (usize, usize),
    result: &mut Vec<Direction>,
) -> (usize, usize) {
    if pos.1 > 0 {
        swap_tile(tiles, (pos.0, pos.1), (pos.0, pos.1 - 1));
        result.push(Direction::Left);
        (pos.0, pos.1 - 1)
    } else {
        pos
    }
}

fn move_up(
    _n: usize,
    tiles: &mut [Vec<u8>],
    pos: (usize, usize),
    result: &mut Vec<Direction>,
) -> (usize, usize) {
    if pos.0 > 0 {
        swap_tile(tiles, (pos.0, pos.1), (pos.0 - 1, pos.1));
        result.push(Direction::Up);
        (pos.0 - 1, pos.1)
    } else {
        pos
    }
}

fn move_right(
    n: usize,
    tiles: &mut [Vec<u8>],
    pos: (usize, usize),
    result: &mut Vec<Direction>,
) -> (usize, usize) {
    if pos.1 + 1 < n {
        swap_tile(tiles, (pos.0, pos.1), (pos.0, pos.1 + 1));
        result.push(Direction::Right);
        (pos.0, pos.1 + 1)
    } else {
        pos
    }
}

fn move_down(
    n: usize,
    tiles: &mut [Vec<u8>],
    pos: (usize, usize),
    result: &mut Vec<Direction>,
) -> (usize, usize) {
    if pos.0 + 1 < n {
        swap_tile(tiles, (pos.0, pos.1), (pos.0 + 1, pos.1));
        result.push(Direction::Down);
        (pos.0 + 1, pos.1)
    } else {
        pos
    }
}

fn apply_route(
    n: usize,
    tiles: &mut [Vec<u8>],
    used: &mut [Vec<i32>],
    pos_0: (usize, usize),
    pos_from: (usize, usize),
    pos_to: (usize, usize),
    result: &mut Vec<Direction>,
) -> ((usize, usize), (usize, usize)) {
    used[pos_from.0][pos_from.1] += 1;
    let prev = calc_prev(n, used, pos_0);
    used[pos_from.0][pos_from.1] -= 1;

    let mut route = Vec::new();
    route.push(dir_from_pos(pos_to, pos_from));
    if pos_to.0 == n {
        return (pos_0, pos_to);
    }

    let mut curr = pos_to;
    while curr.0 != n {
        route.push(dir_from_pos(prev[curr.0][curr.1], curr));
        curr = prev[curr.0][curr.1];
    }
    route.reverse();

    let mut pos_0 = pos_0;
    for dir in route {
        match dir {
            Direction::None => {}
            Direction::Left => {
                pos_0 = move_left(n, tiles, pos_0, result);
            }
            Direction::Up => {
                pos_0 = move_up(n, tiles, pos_0, result);
            }
            Direction::Right => {
                pos_0 = move_right(n, tiles, pos_0, result);
            }
            Direction::Down => {
                pos_0 = move_down(n, tiles, pos_0, result);
            }
        }
    }

    (pos_0, pos_to)
}

fn tile_move_left(
    n: usize,
    tiles: &mut [Vec<u8>],
    used: &mut [Vec<i32>],
    pos_0: (usize, usize),
    pos_from: (usize, usize),
    result: &mut Vec<Direction>,
) -> ((usize, usize), (usize, usize)) {
    if pos_from.1 > 0 {
        let pos_to = (pos_from.0, pos_from.1 - 1);
        apply_route(n, tiles, used, pos_0, pos_from, pos_to, result)
    } else {
        (pos_0, pos_from)
    }
}

fn tile_move_up(
    n: usize,
    tiles: &mut [Vec<u8>],
    used: &mut [Vec<i32>],
    pos_0: (usize, usize),
    pos_from: (usize, usize),
    result: &mut Vec<Direction>,
) -> ((usize, usize), (usize, usize)) {
    if pos_from.0 > 0 {
        let pos_to = (pos_from.0 - 1, pos_from.1);
        apply_route(n, tiles, used, pos_0, pos_from, pos_to, result)
    } else {
        (pos_0, pos_from)
    }
}

fn tile_move_right(
    n: usize,
    tiles: &mut [Vec<u8>],
    used: &mut [Vec<i32>],
    pos_0: (usize, usize),
    pos_from: (usize, usize),
    result: &mut Vec<Direction>,
) -> ((usize, usize), (usize, usize)) {
    if pos_from.1 < n {
        let pos_to = (pos_from.0, pos_from.1 + 1);
        apply_route(n, tiles, used, pos_0, pos_from, pos_to, result)
    } else {
        (pos_0, pos_from)
    }
}

fn tile_move_down(
    n: usize,
    tiles: &mut [Vec<u8>],
    used: &mut [Vec<i32>],
    pos_0: (usize, usize),
    pos_from: (usize, usize),
    result: &mut Vec<Direction>,
) -> ((usize, usize), (usize, usize)) {
    if pos_from.0 < n {
        let pos_to = (pos_from.0 + 1, pos_from.1);
        apply_route(n, tiles, used, pos_0, pos_from, pos_to, result)
    } else {
        (pos_0, pos_from)
    }
}

fn tile_navigate_to(
    n: usize,
    tiles: &mut [Vec<u8>],
    used: &mut [Vec<i32>],
    pos_0: (usize, usize),
    pos_from: (usize, usize),
    pos_to: (usize, usize),
    result: &mut Vec<Direction>,
) -> (usize, usize) {
    let prev = calc_prev(n, used, pos_from);
    let mut route = Vec::new();
    let mut curr = pos_to;
    while curr.0 != n {
        route.push(dir_from_pos(prev[curr.0][curr.1], curr));
        curr = prev[curr.0][curr.1];
    }
    route.reverse();

    let mut pos = (pos_0, pos_from);
    for dir in route {
        match dir {
            Direction::None => {}
            Direction::Left => {
                pos = tile_move_left(n, tiles, used, pos.0, pos.1, result);
            }
            Direction::Up => {
                pos = tile_move_up(n, tiles, used, pos.0, pos.1, result);
            }
            Direction::Right => {
                pos = tile_move_right(n, tiles, used, pos.0, pos.1, result);
            }
            Direction::Down => {
                pos = tile_move_down(n, tiles, used, pos.0, pos.1, result);
            }
        }
    }

    pos.0
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
                        dsu.merge(a, b);
                        tree[dsu.leader_of(a)] = tree[a] && tree[b];
                    }
                }
                if cw + 1 < n && self.tiles[ch][cw] & 4 != 0 && self.tiles[ch][cw + 1] & 1 != 0 {
                    let a = dsu.leader_of(ch * n + cw);
                    let b = dsu.leader_of(ch * n + cw + 1);
                    if a == b {
                        tree[a] = false;
                    } else {
                        dsu.merge(a, b);
                        tree[dsu.leader_of(a)] = tree[a] && tree[b];
                    }
                }
            }
        }

        let mut max_tree = 0;
        for ch in 0..n {
            for cw in 0..n {
                let u = ch * n + cw;
                if self.tiles[ch][cw] != 0 && tree[dsu.leader_of(u)] {
                    max_tree = max(max_tree, dsu.size_of(u));
                }
            }
        }

        max_tree as f64
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
        self.output.directions.len() <= self.input.t
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
    Left,
    Up,
    Right,
    Down,
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::None => '.',
            Direction::Left => 'L',
            Direction::Up => 'U',
            Direction::Right => 'R',
            Direction::Down => 'D',
        }
    }
}

impl Into<(i32, i32)> for Direction {
    fn into(self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
        }
    }
}

impl From<(i32, i32)> for Direction {
    fn from((h, w): (i32, i32)) -> Self {
        let h = h.min(1).max(-1);
        let w = w.min(1).max(-1);
        match (h, w) {
            (0, -1) => Direction::Left,
            (-1, 0) => Direction::Up,
            (0, 1) => Direction::Right,
            (1, 0) => Direction::Down,
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
