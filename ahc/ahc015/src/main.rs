#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{BufReader, StdinLock};
use std::mem::*;

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::source::line::LineSource;
use proconio::*;
use rand::rngs::StdRng;
use rand::Rng;

const H: usize = 10;
const W: usize = 10;
const N: usize = 100;
const DIR: [Direction; 4] = [
    Direction::Front,
    Direction::Back,
    Direction::Left,
    Direction::Right,
];

fn main() {
    let stdin = io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        f: [usize; N],
    }

    let mut g = vec![vec![0; W]; H];
    let mut d = vec![0; 4];
    for t in 0..N {
        input! {
            from &mut source,
            p: usize,
        }

        let p = get_point(&g, p);
        g[p.0][p.1] = f[t];
        d[f[t]] += 1;
        let dir = best_direction(&g, &d);
        g = tilt(&g, dir);
        println!("{}", dir);
    }
}

pub fn tilt(g: &[Vec<usize>], dir: Direction) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; W]; H];
    for i in 0..H {
        for j in 0..W {
            result[i][j] = g[i][j];
        }
    }

    match dir {
        Direction::Front => {
            for i in 0..H {
                for j in 0..W {
                    let mut h = i;
                    while h > 0 && result[h - 1][j] == 0 {
                        result[h - 1][j] = result[h][j];
                        result[h][j] = 0;
                        h -= 1;
                    }
                }
            }
        }
        Direction::Back => {
            for i in (0..H).rev() {
                for j in 0..W {
                    let mut h = i;
                    while h + 1 < H && result[h + 1][j] == 0 {
                        result[h + 1][j] = result[h][j];
                        result[h][j] = 0;
                        h += 1;
                    }
                }
            }
        }
        Direction::Left => {
            for i in 0..H {
                for j in 0..W {
                    let mut w = j;
                    while w > 0 && result[i][w - 1] == 0 {
                        result[i][w - 1] = result[i][w];
                        result[i][w] = 0;
                        w -= 1;
                    }
                }
            }
        }
        Direction::Right => {
            for i in 0..H {
                for j in (0..W).rev() {
                    let mut w = j;
                    while w + 1 < W && result[i][w + 1] == 0 {
                        result[i][w + 1] = result[i][w];
                        result[i][w] = 0;
                        w += 1;
                    }
                }
            }
        }
    }

    result
}

pub fn get_point(g: &[Vec<usize>], p: usize) -> (usize, usize) {
    let mut x = 0;
    for i in 0..H {
        for j in 0..W {
            if g[i][j] == 0 {
                x += 1;
            }

            if x == p {
                return (i, j);
            }
        }
    }

    (0, 0)
}

pub fn best_direction(g: &[Vec<usize>], d: &[usize]) -> Direction {
    let mut scores = vec![0; 4];
    for i in 0..4 {
        scores[i] = compute_score(g, d, DIR[i]);
    }

    let mut result = Direction::Front;
    let mut max = 0;
    for i in 0..4 {
        if max < scores[i] {
            max = scores[i];
            result = DIR[i];
        }
    }

    result
}

pub fn compute_score(g: &[Vec<usize>], d: &[usize], dir: Direction) -> i64 {
    let g = tilt(&g, dir);
    let delta = DeltaIndex::new((H, W), &[(-1, 0), (1, 0), (0, -1), (0, 1)]);
    let mut visited = vec![vec![false; W]; H];
    let mut num = 0;

    for i in 0..H {
        for j in 0..W {
            if !visited[i][j] && g[i][j] != 0 {
                visited[i][j] = true;
                let f = g[i][j];
                let mut size = 1;
                let mut stack = vec![(i, j)];
                while let Some(curr) = stack.pop() {
                    for next in delta.generate(curr) {
                        if !visited[next.0][next.1] && g[next.0][next.1] == f {
                            visited[next.0][next.1] = true;
                            size += 1;
                            stack.push(next);
                        }
                    }
                }
                num += size * size;
            }
        }
    }

    (1e6 * num as f64 / d[1..].iter().map(|d| *d as i64 * *d as i64).sum::<i64>() as f64).round()
        as i64
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Front,
    Back,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Front => 'F',
            Direction::Back => 'B',
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{}", c)
    }
}

struct DeltaIndex<'a> {
    limit: (usize, usize),
    delta: &'a [(i32, i32)],
}

impl<'a> DeltaIndex<'a> {
    fn new(limit: (usize, usize), delta: &'a [(i32, i32)]) -> Self {
        Self { limit, delta }
    }

    fn generate(&self, origin: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
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
