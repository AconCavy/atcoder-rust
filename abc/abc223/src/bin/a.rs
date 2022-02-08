use proconio::*;

#[fastout]
fn main() {
    input! {
        x: i32
    }

    let answer = x != 0 && x % 100 == 0;
    println!("{}", if answer { "Yes" } else { "No" });
}
