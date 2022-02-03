use proconio::*;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut line: [[i64]; n]
    }

    line.sort();
    line.dedup();
    let answer = line.len();
    println!("{}", answer);
}
