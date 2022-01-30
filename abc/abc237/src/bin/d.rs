use itertools::Itertools;
use proconio::marker::Chars;
use proconio::*;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut deq: VecDeque<usize> = VecDeque::new();
    deq.push_front(n);
    for (i, c) in s.iter().enumerate().rev() {
        if *c == 'L' {
            deq.push_back(i);
        } else {
            deq.push_front(i);
        }
    }

    println!("{}", deq.iter().join(" "));
}
