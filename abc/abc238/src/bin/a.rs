use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i32
    }

    let answer = n == 1 || n >= 5;
    println!("{}", if answer { "Yes" } else { "No" });
}
