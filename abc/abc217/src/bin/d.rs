use proconio::*;
use std::collections::BTreeSet;
use std::collections::Bound::{Excluded, Unbounded};

#[fastout]
fn main() {
    input! {
        l: i32,
        q: usize,
    }

    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(l);

    for _ in 0..q {
        input! {
            c: i32,
            x: i32,
        }

        if c == 1 {
            set.insert(x);
        } else {
            let mut prev = set.range((Unbounded, Excluded(x)));
            let mut next = set.range((Excluded(x), Unbounded));
            let answer = next.next().unwrap() - prev.next_back().unwrap();
            println!("{}", answer);
        }
    }
}
