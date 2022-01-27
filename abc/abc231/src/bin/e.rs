use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i64,
        x: i64,
        a:[i64;n],
    }

    let mut dp1: HashMap<i64, i64> = HashMap::new();
    dp1.insert(x, 0);
    let inf = 1e18 as i64;

    for a in a.into_iter().rev() {
        let mut dp2: HashMap<i64, i64> = HashMap::new();
        for (b, c) in &dp1 {
            let upper = (*b + a - 1) / a;
            let rem = num::abs(*b - a * upper);
            let count = std::cmp::min(*dp2.entry(rem).or_insert(inf), *c + upper);
            *dp2.get_mut(&rem).unwrap() = count;

            let lower = b / a;
            let rem = b - a * lower;
            let count = std::cmp::min(*dp2.entry(rem).or_insert(inf), *c + lower);
            *dp2.get_mut(&rem).unwrap() = count;
        }

        std::mem::swap(&mut dp1, &mut dp2);
    }

    let answer = dp1[&0];
    println!("{}", answer);
}
