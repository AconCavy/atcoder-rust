use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[i32; m]; n]
    }

    let mut answer = true;
    for i in 1..n {
        for j in 0..m {
            answer &= b[i][j] == b[i - 1][j] + 7;
        }
    }

    for i in 0..n {
        for j in 1..m {
            answer &= b[i][j] == b[i][j - 1] + 1;
        }
    }

    let mut x = b[0][0] % 7;
    if x == 0 {
        x += 7;
    }
    answer &= x <= 8 - m as i32;
    println!("{}", if answer { "Yes" } else { "No" });
}
