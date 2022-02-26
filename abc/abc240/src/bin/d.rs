use proconio::*;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut stack = VecDeque::new();
    let mut curr = 0;
    for a in a {
        curr += 1;
        if let Some((v, c)) = stack.pop_back() {
            if v == a {
                stack.push_back((v, c + 1));
            } else {
                stack.push_back((v, c));
                stack.push_back((a, 1));
            }
        } else {
            stack.push_back((a, 1));
        }

        while !stack.is_empty() {
            if let Some((v, c)) = stack.pop_back() {
                if v == c {
                    curr -= c;
                } else {
                    stack.push_back((v, c));
                    break;
                }
            } else {
                break;
            }
        }

        println!("{}", curr);
    }
}
