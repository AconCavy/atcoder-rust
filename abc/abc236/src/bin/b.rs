use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; 4 * n - 1],
    }

    let mut answer = 0;
    for v in a {
        answer ^= v;
    }

    println!("{}", answer);
}
