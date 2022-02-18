use proconio::*;

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    let answer = s < t;
    println!("{}", if answer { "Yes" } else { "No" });
}
