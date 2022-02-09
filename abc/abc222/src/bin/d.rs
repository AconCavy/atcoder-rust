use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    }

    let m = 998244353;
    let mut dp = vec![vec![0i64; 3010]; n];
    for j in a[0]..(b[0] + 1) {
        dp[0][j] = 1;
    }

    for i in 1..n {
        let mut s = 0i64;
        let l = std::cmp::min(a[i - 1], a[i]);
        let r = b[i] + 1;
        for j in l..r {
            s += dp[i - 1][j];
            s %= m;
            dp[i][j] += s;
            dp[i][j] %= m;
        }
    }

    let mut answer = 0i64;
    for i in a[n - 1]..(b[n - 1] + 1) {
        answer += dp[n - 1][i];
        answer %= m;
    }

    println!("{}", answer);
}
