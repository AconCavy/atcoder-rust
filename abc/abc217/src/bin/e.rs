use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();
    let mut heap = BinaryHeap::new();

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: i32,
            }

            queue.push_back(x);
        } else if t == 2 {
            if let Some(Reverse(x)) = heap.pop() {
                println!("{}", x);
            } else {
                println!("{}", queue.pop_front().unwrap());
            }
        } else {
            while let Some(x) = queue.pop_front() {
                heap.push(Reverse(x));
            }
        }
    }
}
