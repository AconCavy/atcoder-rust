use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        x:Usize1,
        a:[Usize1;n],
    }

    let mut curr = x;
    let mut used = vec![false; n];
    while !used[curr] {
        used[curr] = true;
        curr = a[curr];
    }

    let answer = used.into_iter().filter(|x| *x).count();
    println!("{}", answer);
}
