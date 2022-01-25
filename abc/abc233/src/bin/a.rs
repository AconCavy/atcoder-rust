use proconio::*;

#[fastout]
fn main() {
    input! {
        x: i32,
        y: i32,
    }

    let answer = std::cmp::max(0, (y - x + 10 - 1) / 10);
    println!("{}", answer)
}
