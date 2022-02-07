use proconio::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h]
    }

    let mut answer = true;
    for i1 in 0..h {
        for i2 in (i1 + 1)..h {
            for j1 in 0..w {
                for j2 in (j1 + 1)..w {
                    answer &= a[i1][j1] + a[i2][j2] <= a[i2][j1] + a[i1][j2];
                }
            }
        }
    }
    println!("{}", if answer { "Yes" } else { "No" });
}
