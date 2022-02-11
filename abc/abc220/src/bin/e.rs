use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize
    }

    let m = 998244353;
    let mut p2 = vec![0i64; n.max(d) + 1];
    p2[0] = 1;
    for i in 1..p2.len() {
        p2[i] = p2[i - 1] * 2 % m;
    }

    let mut answer = 0;
    if n as i32 - d as i32 >= 0 {
        answer += (p2[n - d] - 1) * p2[d];
        answer %= m;
    }

    for l in 1..d {
        let r = d - l;
        let k = l.max(r);
        if n as i32 - k as i32 >= 0 {
            answer += (p2[n - k] - 1) * p2[d - 2];
            answer %= m;
        }
    }

    answer *= 2;
    answer %= m;

    println!("{}", answer);
}
