use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut answer = vec![0; n];
    for (i, p) in p.into_iter().enumerate() {
        answer[p] = i + 1;
    }

    println!("{}", answer.into_iter().join(" "));
}
