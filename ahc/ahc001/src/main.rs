use itertools::Itertools;
use proconio::*;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

#[fastout]
pub fn main() {
    input! {
        n: usize,
    }

    let mut requests = Vec::with_capacity(n);
    let mut ads = Vec::with_capacity(n);

    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
            area: i64,
        }

        requests.push(Request::new(x, y, area));
        ads.push(Ad::new(x, y, x + 1, y + 1));
    }

    const TIME_LIMIT: u64 = 4950;
    const START_TEMP: f64 = 1e-7;
    const END_TEMP: f64 = 1e-5;
    const EXPANSION: u64 = 2000;
    let simulator = AnnealingSimulator::new(EXPANSION, START_TEMP, END_TEMP, 0);
    let state = State::new(n, requests, ads, true, DirType::D4);
    let mut expanded = simulator.simulate(state);

    let simulator = AnnealingSimulator::new(TIME_LIMIT - EXPANSION, START_TEMP, END_TEMP, 1);
    expanded.dir_type = DirType::D12;
    let result = simulator.simulate(expanded);
    // println!("\n{}", (result.calc_score() * 1e9) as i64);
    println!("{}", result.ads.into_iter().join("\n"));
}

#[derive(Copy, Clone)]
pub enum DirType {
    D4 = 4,
    D8 = 8,
    D12 = 12,
}

impl DirType {
    const DIR: [(i64, i64, i64, i64); 12] = [
        (-1, 0, 0, 0),
        (0, -1, 0, 0),
        (0, 0, 1, 0),
        (0, 0, 0, 1),
        (-1, 0, -1, 0),
        (0, -1, 0, -1),
        (1, 0, 1, 0),
        (0, 1, 0, 1),
        (1, 0, 0, 0),
        (0, 1, 0, 0),
        (0, 0, -1, 0),
        (0, 0, 0, -1),
    ];
}

#[derive(Clone)]
pub struct State {
    n: usize,
    requests: Vec<Request>,
    ads: Vec<Ad>,
    is_valid: bool,
    dir_type: DirType,
}

impl State {
    pub fn new(
        n: usize,
        requests: Vec<Request>,
        ads: Vec<Ad>,
        is_valid: bool,
        dir_type: DirType,
    ) -> Self {
        Self {
            n,
            requests,
            ads,
            is_valid,
            dir_type,
        }
    }

    fn calc(request: &Request, ad: &Ad) -> f64 {
        if ad.contains(request) {
            let s = ad.area();
            let p = 1.0 - s.min(request.area) as f64 / s.max(request.area) as f64;
            1.0 - p * p
        } else {
            0.0
        }
    }

    fn calc_all(requests: &Vec<Request>, ads: &Vec<Ad>) -> f64 {
        requests
            .iter()
            .zip(ads.iter())
            .map(|(r, a)| State::calc(r, a))
            .fold(0.0, |acc, x| acc + x)
            / ads.len() as f64
    }
}

impl AnnealingState for State {
    fn calc_score(&self) -> f64 {
        State::calc_all(&self.requests, &self.ads)
    }

    fn modify(&self, rng: &mut rand::rngs::StdRng) -> Self {
        let idx = rng.gen_range(0, self.n);
        let dir = rng.gen_range(0, self.dir_type as usize);
        let mut modified = self.ads.clone();
        let (dx0, dy0, dx1, dy1) = DirType::DIR[dir];
        modified[idx].arrange(dx0, dy0, dx1, dy1);
        let mut ok = modified[idx].is_valid();
        for (i, other) in self.ads.iter().enumerate() {
            if !ok {
                break;
            }
            if i != idx {
                ok &= !modified[idx].intersect(other);
            }
        }

        State::new(self.n, self.requests.clone(), modified, ok, self.dir_type)
    }

    fn is_valid(&self) -> bool {
        self.is_valid
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn translate(&mut self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    p0: Point,
    p1: Point,
}

impl Rect {
    pub fn contains(&self, p: &Point) -> bool {
        self.p0.x <= p.x && p.x < self.p1.x && self.p0.y <= p.y && p.y < self.p1.y
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.p0.x.max(other.p0.x) < self.p1.x.min(other.p1.x)
            && self.p0.y.max(other.p0.y) < self.p1.y.min(other.p1.y)
    }

    pub fn area(&self) -> i64 {
        (self.p1.x - self.p0.x) * (self.p1.y - self.p0.y)
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.p0, self.p1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Request {
    p: Point,
    area: i64,
}

impl Request {
    pub fn new(x: i64, y: i64, area: i64) -> Self {
        Self {
            p: Point { x, y },
            area,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ad {
    rect: Rect,
}

impl Ad {
    const MIN: i64 = 0;
    const MAX: i64 = 10000;

    pub fn new(x0: i64, y0: i64, x1: i64, y1: i64) -> Self {
        Self {
            rect: Rect {
                p0: Point { x: x0, y: y0 },
                p1: Point { x: x1, y: y1 },
            },
        }
    }

    pub fn is_valid(&self) -> bool {
        Ad::MIN <= self.rect.p0.x
            && self.rect.p0.x <= Ad::MAX
            && Ad::MIN <= self.rect.p0.y
            && self.rect.p0.y <= Ad::MAX
            && Ad::MIN <= self.rect.p1.x
            && self.rect.p1.x <= Ad::MAX
            && Ad::MIN <= self.rect.p1.y
            && self.rect.p1.y <= Ad::MAX
            && self.rect.p0.x < self.rect.p1.x
            && self.rect.p0.y < self.rect.p1.y
    }

    pub fn contains(&self, req: &Request) -> bool {
        self.rect.contains(&req.p)
    }

    pub fn intersect(&self, other: &Ad) -> bool {
        self.rect.intersect(&other.rect)
    }

    pub fn area(&self) -> i64 {
        self.rect.area()
    }

    // pub fn arrange(&mut self, dir: usize) {
    pub fn arrange(&mut self, dx0: i64, dy0: i64, dx1: i64, dy1: i64) {
        // let (dx1, dy1, dx2, dy2) = Self::DIR[dir];
        self.rect.p0.translate(dx0, dy0);
        self.rect.p1.translate(dx1, dy1);
    }
}

impl Display for Ad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rect)
    }
}

pub trait AnnealingState {
    fn calc_score(&self) -> f64;
    fn modify(&self, rng: &mut rand::rngs::StdRng) -> Self;
    fn is_valid(&self) -> bool;
}

pub struct AnnealingSimulator {
    time_limit_millis: u64,
    start_temp: f64,
    end_temp: f64,
    random_seed: u64,
}

impl AnnealingSimulator {
    pub fn new(time_limit_millis: u64, start_temp: f64, end_temp: f64, random_seed: u64) -> Self {
        Self {
            time_limit_millis,
            start_temp,
            end_temp,
            random_seed,
        }
    }

    pub fn simulate<S: AnnealingState + Clone>(&self, init_state: S) -> S {
        let timer = Instant::now();
        let end = Duration::from_millis(self.time_limit_millis);
        let mut curr_state = init_state;
        let mut curr_score = curr_state.calc_score();
        let mut best_state = curr_state.clone();
        let mut best_score = curr_score;
        let mut rng_prob: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(self.random_seed);
        let mut rng_modify: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(self.random_seed);

        let calc_temp = |elapsed: f64| -> f64 {
            self.start_temp
                + (self.end_temp - self.start_temp) * elapsed / self.time_limit_millis as f64
        };

        let calc_prob = |score_from: f64, score_to: f64, temp: f64| -> f64 {
            let delta = (score_to - score_from).min(0.0) as f64;
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
