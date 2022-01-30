use proconio::*;

#[fastout]
fn main() {
    input! {
        n:i64,
    }

    let inf = 1i64 << 31;
    let answer = -inf <= n && n < inf;
    println!("{}", if answer { "Yes" } else { "No" });
}
