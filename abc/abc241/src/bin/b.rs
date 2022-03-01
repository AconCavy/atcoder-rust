use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; m],
    }

    let mut count = HashMap::new();
    for a in &a {
        *count.entry(a).or_insert(0) += 1;
    }

    for b in &b {
        if *count.entry(b).or_insert(0) == 0 {
            println!("No");
            return;
        }

        *count.entry(b).or_insert(0) -= 1;
    }

    println!("Yes");
}
