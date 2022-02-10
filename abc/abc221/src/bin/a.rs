use proconio::*;

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32
    }

    let answer = 32i64.pow((a - b) as u32);
    println!("{}", answer)
}
