use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i64,
        k: i64,
        a:[i64;n],
    }

    let mut ng = 1e18 as i64 / k;
    let mut ok = 0;
    while (ok - ng).abs() > 1 {
        let m = (ok + ng) / 2;
        let mut sum = 0i64;
        for a in &a {
            sum += std::cmp::min(*a, m);
        }

        if sum >= m * k {
            ok = m;
        } else {
            ng = m;
        }
    }

    println!("{}", ok);
}
