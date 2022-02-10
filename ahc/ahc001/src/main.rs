use itertools::Itertools;
use proconio::*;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut requests: Vec<Request> = Vec::with_capacity(n);
    let mut ads: Vec<Ad> = Vec::with_capacity(n);
    let mut scores = vec![0.0; n];

    for i in 0..n {
        input! {
            x: i64,
            y: i64,
            area: i64,
        }
        let request = Request::new(x, y, area);
        let ad = Ad::new(x, y, x + 1, y + 1);
        scores[i] = calc(&request, &ad);
        requests.push(request);
        ads.push(ad);
    }

    let timer = Instant::now();
    let end = Duration::from_millis(4900);
    while timer.elapsed() < end {
        let idx = rand::thread_rng().gen_range(0, n);
        let dir = rand::thread_rng().gen_range(0, 4);

        let mut next = ads[idx].clone();
        next.arrange(dir);
        let mut ok = next.is_valid();

        for (i, ad) in ads.iter().enumerate() {
            if !ok {
                break;
            }

            if i != idx {
                ok &= !next.intersect(&ad);
            }
        }

        if ok {
            let score = calc(&requests[idx], &next);
            if scores[idx] <= score {
                scores[idx] = score;
                ads[idx] = next;
            }
        }
    }

    println!("{}", ads.into_iter().join("\n"));
}

fn calc(request: &Request, ad: &Ad) -> f64 {
    if !ad.contains(request) {
        return 0.0;
    }

    let s = ad.area();
    let p = 1.0 - s.min(request.area) as f64 / s.max(request.area) as f64;
    1.0 - p * p
}

fn calc_all(requests: &Vec<Request>, ads: &Vec<Ad>) -> f64 {
    assert_eq!(requests.len(), ads.len());

    let sum = requests
        .iter()
        .zip(ads.iter())
        .map(|(r, a)| calc(r, a))
        .sum::<f64>();
    sum / ads.len() as f64 * 1e9
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn translate(&mut self, dx: i64, dy: i64) {
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
struct Rect {
    p1: Point,
    p2: Point,
}

impl Rect {
    fn contains(&self, p: &Point) -> bool {
        self.p1.x <= p.x && p.x <= self.p2.x && self.p1.y <= p.y && p.y <= self.p2.y
    }

    fn intersect(&self, other: &Rect) -> bool {
        self.p1.x <= other.p2.x
            && other.p1.x <= self.p2.x
            && self.p1.y <= other.p2.y
            && other.p1.y <= self.p2.y
    }

    fn area(&self) -> i64 {
        (self.p2.x - self.p1.x) * (self.p2.y - self.p1.y)
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.p1, self.p2)
    }
}

#[derive(PartialEq)]
struct Request {
    p: Point,
    area: i64,
}

impl Request {
    fn new(x: i64, y: i64, area: i64) -> Self {
        Self {
            p: Point { x, y },
            area,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Ad {
    rect: Rect,
}

impl Ad {
    fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        Self {
            rect: Rect {
                p1: Point { x: x1, y: y1 },
                p2: Point { x: x2, y: y2 },
            },
        }
    }

    fn is_valid(&self) -> bool {
        let min = 0;
        let max = 10000;
        min <= self.rect.p1.x
            && self.rect.p1.x <= max
            && min <= self.rect.p1.y
            && self.rect.p1.y <= max
            && min <= self.rect.p2.x
            && self.rect.p2.x <= max
            && min <= self.rect.p2.y
            && self.rect.p2.y <= max
            && self.rect.p1.x <= self.rect.p2.x
            && self.rect.p1.y <= self.rect.p2.y
    }

    fn contains(&self, req: &Request) -> bool {
        self.rect.contains(&req.p)
    }

    fn intersect(&self, other: &Ad) -> bool {
        self.rect.intersect(&other.rect)
    }

    fn area(&self) -> i64 {
        self.rect.area()
    }

    const DIR: [(i64, i64, i64, i64); 4] = [
        (-1, 0, 0, 0),
        (0, -1, 0, 0),
        (0, 0, 1, 0),
        (0, 0, 0, 1),
        // (-1, 0, -1, 0),
        // (0, -1, 0, -1),
        // (1, 0, 1, 0),
        // (0, 1, 0, 1),
    ];
    fn arrange(&mut self, dir: usize) {
        assert!(dir < 4);
        let (dx1, dy1, dx2, dy2) = Self::DIR[dir];
        self.rect.p1.translate(dx1, dy1);
        self.rect.p2.translate(dx2, dy2);
    }
}

impl Display for Ad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rect)
    }
}
