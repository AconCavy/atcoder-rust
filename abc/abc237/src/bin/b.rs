use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
    }

    let mut b = vec![vec![0; h]; w];
    for i in 0..h {
        input! {
            a:[i32; w]
        }

        for (j, a) in a.iter().enumerate() {
            b[j][i] = *a;
        }
    }

    for b in b {
        println!("{}", b.iter().join(" "));
    }
}
