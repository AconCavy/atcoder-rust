use itertools::Itertools;
use proconio::marker::Chars;
use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        x: Chars,
        n: usize,
        mut s: [Chars; n],
    }

    let map: HashMap<char, usize> = x.iter().enumerate().map(|(i, x)| (*x, i)).collect();
    s.sort_by_key(|v| v.iter().map(|x| map[x]).collect_vec());
    println!("{}", s.into_iter().map(|x| x.iter().join("")).join("\n"));
}
