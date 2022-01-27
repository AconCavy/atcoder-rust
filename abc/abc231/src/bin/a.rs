use proconio::*;

#[fastout]
fn main() {
    input! {
        d: f64,
    }

    let answer = d / 100.0;
    println!("{}", answer);
}
