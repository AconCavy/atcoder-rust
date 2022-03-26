#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use rand::Rng;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::mem::*;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

#[fastout]
fn main() {
    const H: usize = 20;
    const W: usize = 20;
    input! {
        s: (usize, usize),
        t: (usize, usize),
        p: f64,
    }

    let s = Vector2D::new(s.0 as i32, s.1 as i32);
    let t = Vector2D::new(t.0 as i32, t.1 as i32);

    let mut gw = Grid::new(H, W, false);
    let mut gh = Grid::new(H, W, false);
    for k in 0..2 {
        for h in 0..(H - k) {
            input! {
                line: Bytes,
            }

            for (w, v) in line.iter().map(|&x| x == b'0').enumerate() {
                if k == 0 {
                    gw[(h, w)] = v;
                } else {
                    gh[(h, w)] = v;
                }
            }
        }
    }

    const INF: i32 = 1e9 as i32;
    let mut dp = Grid::new(H, W, INF);
    let mut prev = Grid::new(H, W, Vector2D::new(INF, INF));

    dp[s] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(s);
    let d4 = vec![
        Vector2D::new(0, 1),
        Vector2D::new(1, 0),
        Vector2D::new(0, -1),
        Vector2D::new(-1, 0),
    ];

    while let Some(u) = queue.pop_front() {
        // R
        let v = u + d4[0];
        if v.w < W as i32 && gw[u] && dp[v] == INF {
            dp[v] = dp[u] + 1;
            prev[v] = u;
            queue.push_back(v);
        }

        // D
        let v = u + d4[1];
        if v.h < H as i32 && gh[u] && dp[v] == INF {
            dp[v] = dp[u] + 1;
            prev[v] = u;
            queue.push_back(v);
        }

        // L
        let v = u + d4[2];
        if v.w >= 0 && gw[v] && dp[v] == INF {
            dp[v] = dp[u] + 1;
            prev[v] = u;
            queue.push_back(v);
        }

        // U
        let v = u + d4[3];
        if v.h >= 0 && gh[v] && dp[v] == INF {
            dp[v] = dp[u] + 1;
            prev[v] = u;
            queue.push_back(v);
        }
    }

    let mut curr = t;
    let mut route = Vec::with_capacity(200);
    loop {
        let delta = curr - prev[curr];
        let dir = match delta {
            Vector2D { h: 0, w: 1 } => Direction::R,
            Vector2D { h: 1, w: 0 } => Direction::D,
            Vector2D { h: 0, w: -1 } => Direction::L,
            Vector2D { h: -1, w: 0 } => Direction::U,
            _ => Direction::None,
        };
        route.push(dir);
        curr = prev[curr];
        if curr == s {
            break;
        }
    }

    route.reverse();
    let t = ((1.0 / p).floor() as usize).max(1).min(3);
    let mut route2 = Vec::with_capacity(200);
    for &dir in &route {
        for _ in 0..t {
            route2.push(dir);
            let pp = rand::thread_rng().gen::<f64>();
            if pp < 0.10 {
                route2.push(Direction::L);
            } else if pp < 0.20 {
                route2.push(Direction::U);
            }
        }
    }
    println!(
        "{}",
        route2.iter().take(200).map(|x| char::from(*x)).join("")
    );
}

#[derive(Copy, Clone, PartialEq)]
struct Vector2D {
    h: i32,
    w: i32,
}

impl Vector2D {
    fn new(h: i32, w: i32) -> Self {
        Self { h, w }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.h + rhs.h, self.w + rhs.w)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.h += rhs.h;
        self.w += rhs.w;
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.h - rhs.h, self.w - rhs.w)
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.h -= rhs.h;
        self.w -= rhs.w;
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    None,
    U,
    D,
    L,
    R,
}

impl Direction {
    fn inv_char(&self) -> char {
        match self {
            Direction::U => Direction::D.into(),
            Direction::D => Direction::U.into(),
            Direction::L => Direction::R.into(),
            Direction::R => Direction::L.into(),
            _ => '_',
        }
    }
}

impl From<Direction> for char {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::U => 'U',
            Direction::D => 'D',
            Direction::L => 'L',
            Direction::R => 'R',
            _ => '.',
        }
    }
}

struct Grid<T>
where
    T: Clone,
{
    h: usize,
    w: usize,
    values: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn new(h: usize, w: usize, e: T) -> Self {
        Self {
            h,
            w,
            values: vec![e; h * w],
        }
    }

    fn fill(&mut self, value: T) {
        for i in 0..self.values.len() {
            self.values[i] = value.clone();
        }
    }
}

impl<T: std::clone::Clone> Index<Vector2D> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vector2D) -> &Self::Output {
        &self.values[index.h as usize * self.h + index.w as usize]
    }
}

impl<T: std::clone::Clone> IndexMut<Vector2D> for Grid<T> {
    fn index_mut(&mut self, index: Vector2D) -> &mut Self::Output {
        &mut self.values[index.h as usize * self.h + index.w as usize]
    }
}

impl<T: std::clone::Clone> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (h, w): (usize, usize)) -> &Self::Output {
        &self.values[h * self.w + w]
    }
}

impl<T: std::clone::Clone> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (h, w): (usize, usize)) -> &mut Self::Output {
        &mut self.values[(h * self.w + w) as usize]
    }
}
