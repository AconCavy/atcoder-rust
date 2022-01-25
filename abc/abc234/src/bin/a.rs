use proconio::*;

#[fastout]
fn main() {
    input! {
        t: i64,
    }

    let f = |x: i64| return x * x + 2 * x + 3;
    let answer = f(f(f(t) + t) + f(f(t)));
    println!("{}", answer);
}
