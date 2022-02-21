use libm::sqrt;
use proconio::*;

#[fastout]
fn main() {
    input! {
        h: i64,
    }

    let f = |x| sqrt((x * (12800000 + x)) as f64);
    let answer = f(h);
    println!("{}", answer);
}
