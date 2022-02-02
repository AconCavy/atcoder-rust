use proconio::*;

#[fastout]
fn main() {
    input! {
        mut s:i32,
        mut t:i32,
        mut x:i32,
    }
    if s > t {
        t += 24;
    }

    let mut answer = s <= x && x < t;
    answer |= s - 24 <= x && x < t - 24;
    answer |= s + 24 <= x && x < t + 24;
    println!("{}", if answer { "Yes" } else { "No" });
}
