use num_traits::abs;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i64; n],
    }

    let f = |k: i64, a: &[i64]| -> bool {
        return dp(a.into_iter().map(|x| x * 1000 - k).collect()) >= 0;
    };
    let ave = binary_search(1e12 as i64 + 1, 0, &a, f) as f64 / 1000.0;
    println!("{}", ave);

    let f = |k: i64, a: &[i64]| -> bool {
        return dp(a
            .into_iter()
            .map(|x| if *x >= k { 1 } else { -1 })
            .collect())
            > 0;
    };

    let med = binary_search(1e9 as i64 + 1, 0, &a, f);
    println!("{}", med);
}

fn dp(source: Vec<i64>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    for v in source {
        let z = std::cmp::max(x, y) + v;
        x = y;
        y = z;
    }

    return std::cmp::max(x, y);
}

fn binary_search(ng: i64, ok: i64, a: &[i64], f: fn(i64, &[i64]) -> bool) -> i64 {
    let mut ng = ng;
    let mut ok = ok;
    while abs(ng - ok) > 1 {
        let m = (ok + ng) / 2;
        if f(m, a) {
            ok = m;
        } else {
            ng = m;
        }
    }

    return ok;
}
