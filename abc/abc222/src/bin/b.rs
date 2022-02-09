use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: i32,
        a: [i32; n]
    }

    let answer = a.into_iter().filter(|&x| x < p).count();
    println!("{}", answer);
}
