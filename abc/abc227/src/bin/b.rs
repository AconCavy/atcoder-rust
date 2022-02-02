use proconio::*;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s:[i32; n],
    }

    let f = |a: i32, b: i32| return 4 * a * b + 3 * a + 3 * b;
    let mut set: HashSet<i32> = HashSet::new();
    for a in 1..1000 {
        for b in 1..1000 {
            let x = f(a, b);
            if x > 1000 {
                break;
            }
            set.insert(x);
        }
    }

    let mut answer = n;
    for s in s {
        if set.contains(&s) {
            answer -= 1;
        }
    }

    println!("{}", answer);
}
