#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::source::line::LineSource;
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::io::BufReader;
use std::mem::*;

fn main() {
    let stdin = io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    }

    let mut used = vec![false; n * 2 + 2];
    let mut queue = VecDeque::from((1..=(n * 2 + 1)).collect_vec());
    loop {
        while queue.front().map_or(false, |x| used[*x]) {
            queue.pop_front();
        }

        let x = queue.pop_front().unwrap();
        println!("{}", x);

        input! {
            from &mut source,
            y: usize,
        }

        if y == 0 {
            return;
        }

        used[x] = true;
        used[y] = true;
    }
}
