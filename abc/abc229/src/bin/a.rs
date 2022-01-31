use proconio::*;

#[fastout]
fn main() {
    input! {
        s1:String,
        s2:String,
    }

    let answer = s1 == ".#" && s2 == "#." || s1 == "#." && s2 == ".#";
    println!("{}", if answer { "No" } else { "Yes" });
}
