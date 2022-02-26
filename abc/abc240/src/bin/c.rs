use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        input! {
            a: usize,
            b: usize,
        }

        for j in 0..=x {
            if j + a <= x {
                dp[i + 1][j + a] |= dp[i][j];
            }
            if j + b <= x {
                dp[i + 1][j + b] |= dp[i][j];
            }
        }
    }

    let answer = dp[n][x];
    println!("{}", if answer { "Yes" } else { "No" });
}
