use proconio::*;

#[fastout]
fn main() {
    input! {
        abc: i32,
    }

    let a = abc / 100;
    let b = abc / 10 % 10;
    let c = abc % 10;
    let answer = 111 * (a + b + c);
    println!("{}", answer);
}
