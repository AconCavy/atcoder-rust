use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        g: [Chars; n],
    }

    for i in 0..n {
        for j in 0..n {
            let mut ok = false;
            if i + 5 < n {
                let mut count = 0;
                for k in 0..=5 {
                    if g[i + k][j] == '#' {
                        count += 1;
                    }
                }

                ok |= count >= 4;
            }

            if j + 5 < n {
                let mut count = 0;
                for k in 0..=5 {
                    if g[i][j + k] == '#' {
                        count += 1;
                    }
                }

                ok |= count >= 4;
            }

            if i + 5 < n && j + 5 < n {
                let mut count = 0;
                for k in 0..=5 {
                    if g[i + k][j + k] == '#' {
                        count += 1;
                    }
                }

                ok |= count >= 4;
            }

            if i + 5 < n && j as i32 - 5 >= 0 {
                let mut count = 0;
                for k in 0..=5 {
                    if g[i + k][j - k] == '#' {
                        count += 1;
                    }
                }

                ok |= count >= 4;
            }

            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
