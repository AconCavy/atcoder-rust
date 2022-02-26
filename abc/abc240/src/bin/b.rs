use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    a.sort();
    a.dedup();
    let answer = a.len();
    println!("{}", answer);
}
