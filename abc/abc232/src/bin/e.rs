use proconio::*;

#[fastout]
fn main() {
    let m: i64 = 998244353;
    input! {
        h: i64,
        w: i64,
        k: i64,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }

    let mut dp1 = vec![vec![0i64; 2]; 2];
    let same = |v1, v2| -> usize {
        return if v1 == v2 { 1 } else { 0 };
    };
    dp1[same(y1, y2)][same(x1, x2)] = 1;
    for _ in 0..k {
        let mut dp2 = vec![vec![0i64; 2]; 2];
        dp2[1][1] = (dp1[0][1] + dp1[1][0]) % m;
        dp2[1][0] = (dp1[1][1] * (h - 1) % m + dp1[1][0] * (h - 2) % m + dp1[0][0]) % m;
        dp2[0][1] = (dp1[1][1] * (w - 1) % m + dp1[0][1] * (w - 2) % m + dp1[0][0]) % m;
        dp2[0][0] =
            (dp1[1][0] * (w - 1) % m + dp1[0][1] * (h - 1) % m + dp1[0][0] * (h + w - 4) % m) % m;

        std::mem::swap(&mut dp1, &mut dp2);
    }

    let answer = dp1[1][1] % m;
    println!("{}", answer);
}
