use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        k:Usize1,
        p:[[i32; 3]; n]
    }

    let q: Vec<i32> = p.into_iter().map(|x| x.into_iter().sum()).collect();
    let border = *q.iter().sorted().rev().nth(k).unwrap();
    for q in q {
        println!("{}", if border - q <= 300 { "Yes" } else { "No" });
    }
}
