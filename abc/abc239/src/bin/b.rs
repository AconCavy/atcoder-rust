use proconio::*;

#[fastout]
fn main() {
    input! {
        x: i64,
    }

    let answer = if x >= 0 { x / 10 } else { -((-x + 9) / 10) };
    println!("{}", answer);
}
