use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n],
    }

    let inf = 1e9 as i32;
    let mut dp = vec![vec![vec![inf; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = 0;
    for (i, (a, b)) in ab.iter().enumerate() {
        for j in 0..=x {
            for k in 0..=y {
                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
                let nj = (j + a).min(x);
                let nk = (k + b).min(y);
                dp[i + 1][nj][nk] = dp[i + 1][nj][nk].min(dp[i][j][k] + 1);
            }
        }
    }

    let answer = if dp[n][x][y] == inf { -1 } else { dp[n][x][y] };
    println!("{}", answer);
}
