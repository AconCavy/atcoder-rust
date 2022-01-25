use proconio::*;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
        p:[i32;n]
    }

    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    for i in 0..k {
        heap.push(-p[i as usize]);
    }
    println!("{}", -heap.peek().unwrap());

    for i in k..n {
        if let Some(x) = heap.peek() {
            if -(*x) > -p[i as usize] {
                heap.push(-p[i as usize]);
                heap.pop();
            }
        }
        println!("{}", -heap.peek().unwrap());
    }
}
