use proconio::*;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut queue: VecDeque<String> = VecDeque::from(t);
    for v in s {
        let mut answer = "No";
        if let Some(x) = queue.front() {
            if &v == x {
                queue.pop_front();
                answer = "Yes";
            }
        }

        println!("{}", answer);
    }
}
