use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    let mut dp = vec![vec![-1; w]; h];
    dp[0][0] = 1;
    let mut answer = 1;
    let d2 = vec![(0, 1), (1, 0)];
    for ch in 0..h {
        for cw in 0..w {
            if dp[ch][cw] < 0 {
                continue;
            }

            for (dh, dw) in &d2 {
                let (nh, nw) = (ch + dh, cw + dw);
                if nh >= h || nw >= w {
                    continue;
                }

                if c[nh][nw] == '#' {
                    continue;
                }

                dp[nh][nw] = std::cmp::max(dp[nh][nw], dp[ch][cw] + 1);
                answer = std::cmp::max(answer, dp[nh][nw]);
            }
        }
    }

    println!("{}", answer);
}
