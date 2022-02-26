use proconio::*;

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let answer = match (a - b).abs() {
        1 | 9 => true,
        _ => false,
    };
    println!("{}", if answer { "Yes" } else { "No" });
}
