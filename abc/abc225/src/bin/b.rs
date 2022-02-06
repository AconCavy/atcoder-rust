use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut count = vec![0; n];
    for _ in 0..(n - 1) {
        input! {
            a: Usize1,
            b: Usize1
        }

        count[a] += 1;
        count[b] += 1;
    }

    let answer = count.contains(&(n - 1));
    println!("{}", if answer { "Yes" } else { "No" });
}
