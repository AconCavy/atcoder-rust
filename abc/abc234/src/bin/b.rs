use libm::sqrt;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut p: Vec<Point> = Vec::with_capacity(n);
    for _ in 0..n {
        input! { x: i64, y:i64,}
        p.push(Point::new(x, y));
    }

    let mut max: i64 = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            max = std::cmp::max(max, p[i].dist2(&p[j]));
        }
    }

    let answer = sqrt(max as f64);
    println!("{}", answer);
}

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn dist2(&self, other: &Point) -> i64 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        return dx * dx + dy * dy;
    }
}
