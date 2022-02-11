use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let m = 998244353;
    let mut dp = vec![vec![0; 10]; n + 1];
    dp[1][a[0]] = 1;
    for i in 1..n {
        for j in 0..10 {
            dp[i + 1][(j + a[i]) % 10] += dp[i][j];
            dp[i + 1][(j + a[i]) % 10] %= m;
            dp[i + 1][(j * a[i]) % 10] += dp[i][j];
            dp[i + 1][(j * a[i]) % 10] %= m;
        }
    }

    println!("{}", dp[n].iter().join("\n"));
}
