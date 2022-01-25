use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut map: HashMap<i64, i64> = HashMap::new();
    map.insert(0, 1);
    let mut sum: i64 = 0;
    let mut answer = 0;
    for a in a {
        sum += a;
        answer += map.get(&(sum - k)).map_or(0, |v| *v);
        *map.entry(sum).or_insert(0) += 1;
    }

    println!("{}", answer);
}
