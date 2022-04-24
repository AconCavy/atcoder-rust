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
use std::fmt::{Display, Formatter};
use std::io;
use std::mem::*;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use std::time::{Duration, Instant};

const TIME_LIMIT: u64 = 2000 - 50;
const H: usize = 30;
const W: usize = 30;
const HW: usize = H * W;

#[fastout]
pub fn main() {
    const START_TEMP: f64 = 1000.0;
    const END_TEMP: f64 = 0.1;

    input! {
        input: [Bytes; H],
    }

    let mut tiles = vec![vec![0; W]; H];
    for i in 0..H {
        for j in 0..W {
            tiles[i][j] = (input[i][j] - b'0') as usize;
        }
    }

    let mut rotates = vec![0; HW];
    let idx = |h, w| h * W + w;
    for ch in 0..H {
        for cw in 0..W {
            if ch < H / 2 {
                if cw <= W / 2 {
                    rotates[idx(ch, cw)] = match tiles[ch][cw] {
                        0 => 2,
                        1 => 1,
                        2 => 0,
                        3 => 3,
                        4 => 0,
                        5 => 1,
                        _ => 0,
                    };
                } else {
                    rotates[idx(ch, cw)] = match tiles[ch][cw] {
                        0 => 1,
                        1 => 0,
                        2 => 3,
                        3 => 2,
                        4 => 1,
                        5 => 0,
                        _ => 0,
                    };
                }
            } else {
                if cw <= W / 2 {
                    rotates[idx(ch, cw)] = match tiles[ch][cw] {
                        0 => 3,
                        1 => 2,
                        2 => 1,
                        3 => 0,
                        4 => 1,
                        5 => 0,
                        _ => 0,
                    };
                } else {
                    rotates[idx(ch, cw)] = match tiles[ch][cw] {
                        0 => 0,
                        1 => 3,
                        2 => 2,
                        3 => 1,
                        4 => 0,
                        5 => 1,
                        _ => 0,
                    };
                }
            }
        }
    }

    let state = State {
        tiles: Rc::new(tiles),
        rotates,
    };

    let simulator = AnnealingSimulator::new(TIME_LIMIT, START_TEMP, END_TEMP);
    let result = simulator.simulate(state, 0);
    println!("{}", result);
}

#[derive(Clone, Eq, PartialEq)]
pub struct State {
    tiles: Rc<Vec<Vec<usize>>>,
    rotates: Vec<usize>,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rotates.iter().join(""))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.calc_score().cmp(&self.calc_score())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const ROTATE: [usize; 8] = [1, 2, 3, 0, 5, 4, 7, 6];
const TO: [[usize; 4]; 8] = [
    // [L, U, R, D]
    [1, 0, !0, !0],
    [3, !0, !0, 0],
    [!0, !0, 3, 2],
    [!0, 2, 1, !0],
    [1, 0, 3, 2],
    [3, 2, 1, 0],
    [2, !0, 0, !0],
    [!0, 3, !0, 1],
];
const DIJ: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

impl AnnealingState for State {
    fn calc_score(&self) -> i64 {
        let mut tiles = vec![vec![0; W]; H];
        for i in 0..H {
            for j in 0..W {
                for _ in 0..self.rotates[i * W + j] {
                    tiles[i][j] = ROTATE[tiles[i][j]];
                }
            }
        }

        let mut ls = vec![];
        let mut used = vec![[[false; 4]; W]; H];
        let mut cycle = vec![[[(0, 0); 4]; W]; H];
        for i in 0..H {
            for j in 0..W {
                for d in 0..4 {
                    if TO[tiles[i][j]][d] != !0 && !used[i][j][d] {
                        let mut i2 = i;
                        let mut j2 = j;
                        let mut d2 = d;
                        let mut length = 0;
                        let mut tmp = vec![];
                        while !used[i2][j2][d2] {
                            if TO[tiles[i2][j2]][d2] == !0 {
                                break;
                            }
                            length += 1;
                            used[i2][j2][d2] = true;
                            tmp.push((i2, j2, d2));
                            d2 = TO[tiles[i2][j2]][d2];
                            used[i2][j2][d2] = true;
                            tmp.push((i2, j2, d2));
                            i2 = usize::try_from(i2 as i32 + DIJ[d2].0).unwrap_or(H);
                            j2 = usize::try_from(j2 as i32 + DIJ[d2].1).unwrap_or(W);
                            if i2 >= H || j2 >= W {
                                break;
                            }
                            d2 = (d2 + 2) % 4;
                        }
                        if (i, j, d) == (i2, j2, d2) {
                            ls.push((length, tmp.clone()));
                            for (i, j, d) in tmp {
                                cycle[i][j][d].0 = length;
                            }
                        }
                    }
                }
            }
        }
        let score = if ls.len() <= 1 {
            0
        } else {
            ls.sort();
            for &(i, j, d) in &ls[ls.len() - 1].1 {
                cycle[i][j][d].1 = 1;
            }
            for &(i, j, d) in &ls[ls.len() - 2].1 {
                cycle[i][j][d].1 = 2;
            }
            ls[ls.len() - 1].0 * ls[ls.len() - 2].0
        };

        score as i64
    }

    fn modify(&self, rng: &mut StdRng) -> Self {
        let mut rotates = self.rotates.clone();
        let idx = rng.gen_range(0, self.rotates.len());
        rotates[idx] = rng.gen_range(0, 4);
        Self {
            tiles: self.tiles.clone(),
            rotates,
        }
    }

    fn is_valid(&self) -> bool {
        self.rotates.iter().all(|&t| t < 4)
    }
}

pub trait AnnealingState {
    fn calc_score(&self) -> i64;
    fn modify(&self, rng: &mut StdRng) -> Self;
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

    pub fn simulate<S: AnnealingState + Clone>(&self, init_state: S, seed: u64) -> S {
        let timer = Instant::now();
        let end = Duration::from_millis(self.time_limit_millis);
        let mut curr_state = init_state;
        let mut curr_score = curr_state.calc_score();
        let mut best_state = curr_state.clone();
        let mut best_score = curr_score;
        let mut rng_prob: StdRng = rand::SeedableRng::seed_from_u64(seed);
        let mut rng_modify: StdRng = rand::SeedableRng::seed_from_u64(seed);

        let calc_temp = |elapsed: f64| -> f64 {
            self.start_temp
                + (self.end_temp - self.start_temp) * elapsed / self.time_limit_millis as f64
        };

        let calc_prob = |score_from: i64, score_to: i64, temp: f64| -> f64 {
            let delta = (score_to - score_from).min(0) as f64;
            (delta / temp).exp()
        };
        while timer.elapsed() < end {
            let next_state = curr_state.modify(&mut rng_modify);
            if next_state.is_valid() {
                let next_score = next_state.calc_score();
                let temp = calc_temp(timer.elapsed().as_millis() as f64);
                if calc_prob(curr_score, next_score, temp) >= rng_prob.gen::<f64>() {
                    curr_state = next_state;
                    curr_score = next_score;

                    if curr_score > best_score {
                        best_state = curr_state.clone();
                        best_score = curr_score;
                    }
                }
            }
        }

        best_state
    }
}
