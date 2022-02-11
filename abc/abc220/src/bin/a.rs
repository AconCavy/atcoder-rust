use proconio::*;

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    }

    let d = (a + c - 1) / c;
    let answer = if c * d <= b { c * d } else { -1 };
    println!("{}", answer);
}
