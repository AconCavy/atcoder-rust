use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut map: HashMap<String, i32> = HashMap::new();
    for _ in 0..n {
        input! {
            s: String,
        }

        *map.entry(s).or_insert(0) += 1;
    }

    let mut max = 0;
    let mut answer = "".to_string();
    for (s, c) in map {
        if max < c {
            max = c;
            answer = s;
        }
    }

    println!("{}", answer);
}
